extern crate regex;

use crate::Program;
use encoding_rs::SHIFT_JIS;
use regex::Regex;
use std::boxed::Box;
use std::fmt::{self, Debug};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::str::FromStr;

enum Input {
    Stdin,
    Reader(Box<dyn Read>),
}

pub struct Sequence {
    input: Input,
    out: Box<dyn Write>,
    err: Option<Box<dyn Write>>,
    programs: Vec<Program>,
}

impl Sequence {
    fn new_with_stderr(
        input: Input,
        out: Box<dyn Write>,
        err: Box<dyn Write>,
        programs: Vec<Program>,
    ) -> Self {
        Sequence {
            input,
            out,
            err: Some(err),
            programs,
        }
    }

    fn new_without_stderr(input: Input, out: Box<dyn Write>, programs: Vec<Program>) -> Self {
        Sequence {
            input,
            out,
            err: None,
            programs,
        }
    }

    pub fn run(mut self) {
        let mut pipe_buf: Option<Vec<u8>> = None;

        for prog in &self.programs {
            let run_result = match pipe_buf {
                None => match &mut self.input {
                    Input::Stdin => prog.run_with_stdin(),
                    Input::Reader(r) => {
                        let mut buf = vec![];
                        r.read_to_end(&mut buf).unwrap();
                        prog.run_with_input(&buf)
                    }
                },
                Some(buf) => prog.run_with_input(&buf),
            };

            match run_result {
                Err(_) => {
                    eprintln!("Could not run command : {:?}", prog);
                    return;
                }
                Ok(o) => {
                    match self.err.as_mut() {
                        Some(err) => err.write_all(&convert_to_utf8_if_sjis(o.stderr)).unwrap(),
                        None => self
                            .out
                            .write_all(&convert_to_utf8_if_sjis(o.stderr))
                            .unwrap(),
                    }
                    let buf = convert_to_utf8_if_sjis(o.stdout);
                    pipe_buf = Some(buf);

                    if o.status.code().unwrap() != 0 {
                        return;
                    }
                }
            }
        }

        self.out.write_all(&pipe_buf.unwrap_or_default()).unwrap();
    }
}

impl Debug for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.programs)
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex_separators = Regex::from_str(r"^(\||1?>>?|2>>?|<|&>>?|2>&1)$").unwrap();
        let words = split_with_space(s);
        let mut progs = vec![];

        let get_args_to_next_sep = |idx| {
            if idx >= words.len() {
                return None;
            }

            let mut end = idx;

            while end < words.len() && !regex_separators.is_match(words[end]) {
                end += 1;
            }

            Some((&words[idx..end], end))
        };

        let mut input = Input::Stdin;
        let mut stdout = None;
        let mut stderr = None;
        let mut err_to_out = false;
        let mut index = 0;

        while index < words.len() {
            let i = if index == 0 { 0 } else { index + 1 };
            match get_args_to_next_sep(i) {
                None => match words[index] {
                    "2>&1" => err_to_out = true,
                    _ => return Err(()),
                },
                Some((args, next)) => {
                    match words[index] {
                        "|" => match parse_to_program(&args) {
                            Some(p) => progs.push(p),
                            None => return Err(()),
                        },
                        _ if index == 0 => match parse_to_program(&args) {
                            Some(p) => progs.push(p),
                            None => return Err(()),
                        },
                        "<" => match create_reader(args) {
                            Err(_) => return Err(()),
                            Ok(r) => input = Input::Reader(r),
                        },
                        ">" | "1>" => match create_writer(args, false) {
                            Err(_) => return Err(()),
                            Ok(w) => stdout = Some(w),
                        },
                        ">>" | "1>>" => match create_writer(args, true) {
                            Err(_) => return Err(()),
                            Ok(w) => stdout = Some(w),
                        },
                        "2>" => match create_writer(args, false) {
                            Err(_) => return Err(()),
                            Ok(w) => stderr = Some(w),
                        },
                        "2>>" => match create_writer(args, true) {
                            Err(_) => return Err(()),
                            Ok(w) => stderr = Some(w),
                        },
                        "2>&1" => err_to_out = true,
                        "&>" => {
                            match create_writer(args, false) {
                                Err(_) => return Err(()),
                                Ok(w) => stderr = Some(w),
                            }
                            err_to_out = true;
                        }
                        _ => return Err(()),
                    }
                    index = next;
                }
            }
        }

        if err_to_out {
            Ok(Sequence::new_without_stderr(
                input,
                stdout.unwrap_or_else(|| Box::new(io::stdout())),
                progs,
            ))
        } else {
            Ok(Sequence::new_with_stderr(
                input,
                stdout.unwrap_or_else(|| Box::new(io::stdout())),
                stderr.unwrap_or_else(|| Box::new(io::stderr())),
                progs,
            ))
        }
    }
}

fn create_reader(args: &[&str]) -> Result<Box<dyn Read>, ()> {
    if args.len() != 1 {
        return Err(());
    }

    match File::open(args[0]) {
        Ok(file) => Ok(Box::new(file)),
        Err(_) => Err(()),
    }
}

fn create_writer(args: &[&str], append: bool) -> Result<Box<dyn Write>, ()> {
    if args.len() != 1 {
        return Err(());
    }

    if args[0] == "/dev/null" {
        return Ok(Box::new(io::sink()));
    }

    let result = if append {
        OpenOptions::new().append(true).open(args[0])
    } else {
        File::create(args[0])
    };

    match result {
        Ok(file) => Ok(Box::new(file)),
        Err(_) => Err(()),
    }
}

fn parse_to_program(words: &[&str]) -> Option<Program> {
    if words.is_empty() {
        None
    } else if words.len() == 1 {
        Some(Program::new(words[0], &[]))
    } else {
        Some(Program::new(words[0], &words[1..words.len()]))
    }
}

fn split_with_space(s: &str) -> Vec<&str> {
    let mut wrapped = false;
    let mut words = vec![];
    let mut start_index = 0;

    for i in 0..s.len() {
        let c = match s.get(i..=i) {
            None => continue,
            Some(x) => x,
        };

        match (c, wrapped) {
            ("\"", _) => wrapped = !wrapped,
            (" ", false) if start_index < i => {
                words.push(get_str_trim_quot(s, start_index, i - 1));
                start_index = i + 1;
            }
            (" ", false) => {
                start_index = i + 1;
            }
            _ => {}
        }
    }

    if start_index < s.len() {
        words.push(get_str_trim_quot(s, start_index, s.len() - 1))
    }

    words
}

fn get_str_trim_quot(s: &str, start: usize, end: usize) -> &str {
    let start = if s.get(start..=start) == Some("\"") {
        start + 1
    } else {
        start
    };

    let end = if s.get(end..=end) == Some("\"") && start < end {
        end - 1
    } else {
        end
    };

    &s[start..=end]
}

fn convert_to_utf8_if_sjis(buf: Vec<u8>) -> Vec<u8> {
    let (res, _, is_err) = SHIFT_JIS.decode(&buf);

    if is_err {
        buf
    } else {
        res.as_bytes().to_vec()
    }
}

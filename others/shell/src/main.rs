extern crate shell;
extern crate encoding_rs;

use std::io::BufRead;
use std::io::Write;
use shell::{Program};
use encoding_rs::SHIFT_JIS;

fn main() {
    let stdin = std::io::stdin();

    loop {
        let mut line = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        stdin.lock().read_line(&mut line).unwrap();

        let cmds = match parse_commands(remove_newline(&line)) {
            Some(cmds) => cmds,
            None => return,
        };

        println!("{:?}", cmds);

        match cmds.iter().nth(0).unwrap().run() {
            Err(_) => {
                println!("Failed to run program.");
                return;
            },
            Ok(output) => {
                if output.status.code().unwrap() != 0 {
                    std::io::stdout().write_all(&convert_to_utf8_if_sjis(output.stderr)).unwrap();
                } else {
                    std::io::stdout().write_all(&convert_to_utf8_if_sjis(output.stdout)).unwrap();
                }
            },
        }
    }
}

fn parse_commands(s: &str) -> Option<Vec<Program>> {
    let mut cmds = vec![];
    let mut wrapped = false;
    let mut cmd_name = None;
    let mut cmd_args = vec![];
    let mut start_index = 0;

    for i in 0..s.len()+2 {
        let c = if i == s.len()+1 {
            "|"
        } else if i == s.len() {
            " "
        } else {
            match s.get(i..=i) {
                None => continue,
                Some(x) => x,
            }
        };

        match (c, wrapped, cmd_name) {
            ("\"", _, _) => wrapped = !wrapped,
            (" ", false, None) => {
                if start_index < i {
                    cmd_name = Some(&s[start_index..i]);
                }
                start_index = i + 1;
            },
            (" ", false, Some(_)) => {
                if start_index < i {
                    let start = if &s[start_index..=start_index] == "\"" {
                        start_index + 1
                    } else {
                        start_index
                    };
                    let end = if &s[(i - 1)..=(i - 1)] == "\"" {
                        i - 1
                    } else {
                        i
                    };
                    cmd_args.push(&s[start..end]);
                }
                start_index = i + 1;
            },
            ("|", false, Some(name)) => {
                cmds.push(Program::new(name, &cmd_args));
                cmd_name = None;
                cmd_args.clear();
                start_index = i + 1;
            },
            ("|", false, None) => return None,
            _ => {},
        }
    }

    Some(cmds)
}

fn remove_newline(s: &str) -> &str {
    if s.ends_with("\r\n") {
        s.get(0..s.len() - 2).unwrap()
    } else if s.ends_with("\n") || s.ends_with("\r") {
        s.get(0..s.len() - 1).unwrap()
    } else {
        s
    }
}

fn convert_to_utf8_if_sjis(buf: Vec<u8>) -> Vec<u8> {
    let (res, _, is_err) = SHIFT_JIS.decode(&buf);

    if is_err {
        buf
    } else {
        res.as_bytes().iter().map(|&x| x).collect::<Vec<_>>()
    }
}

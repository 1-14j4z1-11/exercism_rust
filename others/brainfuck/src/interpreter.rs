use std::io::Read;
use std::io::Write;

use crate::memory::Memory;

pub struct Executor<'a> {
    reader: &'a mut Read,
    writer: &'a mut Write,
}

impl<'a> Executor<'a> {
    pub fn new(reader: &'a mut Read, writer: &'a mut Write) -> Self {
        Executor { reader, writer }
    }

    pub fn execute(&mut self, code: &str) {
        let mut memory = Memory::new();
        let ops = code.chars().collect::<Vec<_>>();
        let mut pc = 0;

        while pc < ops.len() {
            let op = ops[pc];

            match op {
                '>' => memory.next(),
                '<' => memory.prev(),
                '+' => memory.increment(),
                '-' => memory.decrement(),
                '.' => {
                    self.writer.write_all(&[memory.load()]).unwrap();
                }
                ',' => memory.store(self.read_byte()),
                '[' => {
                    if memory.load() == 0 {
                        pc = next_bracket_r(code, pc).unwrap() + 1;
                        continue;
                    }
                }
                ']' => {
                    pc = prev_bracket_l(code, pc).unwrap();
                    continue;
                }
                _ => {}
            }

            pc += 1;
        }
    }

    fn read_byte(&mut self) -> u8 {
        let mut buf = vec![0];

        match self.reader.read(&mut buf) {
            Ok(1) => buf[0],
            _ => panic!(),
        }
    }
}

fn next_bracket_r(code: &str, pos_bracket_l: usize) -> Option<usize> {
    match code.chars().nth(pos_bracket_l) {
        Some('[') => {}
        _ => panic!(),
    };

    let mut inner_brackets = 0;

    for ptr in (pos_bracket_l + 1)..code.len() {
        match code.chars().nth(ptr) {
            Some('[') => inner_brackets += 1,
            Some(']') => {
                if inner_brackets == 0 {
                    return Some(ptr);
                } else {
                    inner_brackets -= 1;
                }
            }
            Some(_) => {}
            None => return None,
        }
    }

    None
}

fn prev_bracket_l(code: &str, pos_bracket_r: usize) -> Option<usize> {
    match code.chars().nth(pos_bracket_r) {
        Some(']') => {}
        _ => panic!(),
    };

    let mut inner_brackets = 0;

    for ptr in (0..(pos_bracket_r - 1)).rev() {
        match code.chars().nth(ptr) {
            Some(']') => inner_brackets += 1,
            Some('[') => {
                if inner_brackets == 0 {
                    return Some(ptr);
                } else {
                    inner_brackets -= 1;
                }
            }
            Some(_) => {}
            None => return None,
        }
    }

    None
}

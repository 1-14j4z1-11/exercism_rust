use std::collections::HashMap;
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
        let bracket_indices = search_bracket_indices(&ops);

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
                        pc = *bracket_indices.get(&pc).unwrap() + 1;
                        continue;
                    }
                }
                ']' => {
                    pc = *bracket_indices.get(&pc).unwrap();
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

fn search_bracket_indices(code: &[char]) -> HashMap<usize, usize> {
    let mut indices = HashMap::new();

    for i in 0..code.len() {
        if code[i] != '[' {
            continue;
        }

        if let Some(other) = next_bracket_r(code, i) {
            indices.insert(i, other);
            indices.insert(other, i);
        }
    }

    indices
}

fn next_bracket_r(code: &[char], pos_bracket_l: usize) -> Option<usize> {
    match code[pos_bracket_l] {
        '[' => {}
        _ => panic!(),
    };

    let mut inner_brackets = 0;

    for (ptr, op) in code.iter().enumerate().skip(pos_bracket_l + 1) {
        match op {
            '[' => inner_brackets += 1,
            ']' => {
                if inner_brackets == 0 {
                    return Some(ptr);
                } else {
                    inner_brackets -= 1;
                }
            }
            _ => {}
        }
    }

    None
}

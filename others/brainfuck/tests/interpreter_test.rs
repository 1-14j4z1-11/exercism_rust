use std::io::Read;
use std::io::Write;

use brainfuck::interpreter;

#[test]
fn hello_world() {
    let code = r"
        >+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++
        ++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>
        ++++++++[<++++>-]<+.[-]++++++++++.";

    testcase(code, "", "Hello World!\n");
}

#[test]
fn echo() {
    let code = r"+[>,.]";
    testcase(code, "Hello world\0", "Hello world\0");
}

#[test]
fn add() {
    // 3 + 2
    let code = r"
        +++>++><<
        >[-<+>]<
        ++++++++++++++++++++++++++++++++++++++++++++++++.";

    testcase(code, "", "5");
}

#[test]
fn multiply() {
    // 4 * 2
    let code = r"
        ++++>++><<
        [-
            >[->>+<<]
            >>[-<+<+>>]
            <<<
        ]>>
        ++++++++++++++++++++++++++++++++++++++++++++++++.";

    testcase(code, "", "8");
}

#[test]
fn fizz_buzz() {
    let code = r"
        ++++++++++++[->++++++>+++++++++>+++++>++++++++++>++++++++++>+++>>>>>>+<<<<<<<<<<<<]>-->--->++
        ++++>--->++>---->>>>+++>+++++>+++[>>>+[-<<[->>+>+<<<]>>>[-<<<+>>>]+<[[-]>-<<[->+>+<<]>>[-<<+>>]+<[[
        -]>-<<<+>->]>[-<<<--------->+++++++++>>>>>+<<<]<]>[-<+++++++[<<+++++++>>-]<++++++++>>]>>>]<<<<<<[<<<
        <]>-[-<<+>+>]<[->+<]+<[[-]>-<]>[->+++<<<<<<<<<.>.>>>..>>+>>]>>-[-<<<+>+>>]<<[->>+<<]+<[[-]>-<]>[->>+
        ++++<<<<<<<<.>.>..>>+>>]<+<[[-]>-<]>[->>>>>[>>>>]<<<<[.<<<<]<]<<.>>>>>>-]";

    testcase(
        code,
        "",
        "1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz ",
    );
}

fn testcase(code: &str, input: &str, exp_output: &str) {
    let mut out_buffer = IOBuffer::new();
    let mut in_buffer = IOBuffer::new_with_buffer(input.as_bytes());
    let mut executor = interpreter::Executor::new(&mut in_buffer, &mut out_buffer);

    executor.execute(code);
    assert_eq!(
        std::str::from_utf8(out_buffer.get_bytes()).unwrap(),
        exp_output
    );
}

struct IOBuffer {
    buffer: Vec<u8>,
    pos: usize,
}

impl IOBuffer {
    pub fn new() -> Self {
        IOBuffer {
            buffer: vec![],
            pos: 0,
        }
    }

    pub fn new_with_buffer(buffer: &[u8]) -> Self {
        IOBuffer {
            buffer: buffer.iter().map(|&x| x).collect::<Vec<_>>(),
            pos: 0,
        }
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Write for IOBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for &x in buf {
            self.pos += 1;
            if self.pos < self.buffer.len() {
                self.buffer[self.pos] = x;
            } else {
                self.buffer.push(x);
                self.pos = self.buffer.len() - 1;
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for IOBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(self.buffer.len() - self.pos, buf.len());

        for i in 0..len {
            buf[i] = self.buffer[self.pos];
            self.pos += 1;
        }

        Ok(len)
    }
}

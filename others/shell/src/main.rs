extern crate encoding_rs;
extern crate shell;

use shell::Sequence;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

fn main() {
    let stdin = std::io::stdin();

    loop {
        let mut line = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        stdin.lock().read_line(&mut line).unwrap();
        let line = remove_newline(&line);

        if line == "exit" {
            break;
        }

        match Sequence::from_str(line) {
            Ok(seq) => seq.run(),
            Err(_) => writeln!(io::stderr(), "Invalid command").unwrap(),
        };
    }
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

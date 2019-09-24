extern crate unix_cmd;

use std::iter::Iterator;
use unix_cmd::*;

fn main() {
    let args_string = std::env::args().collect::<Vec<_>>();
    let args = args_string.iter().map(|a| &**a).collect::<Vec<&str>>();

    if args.len() <= 1 {
        return;
    }

    match args[1] {
        "ls" => ls::run(&args[2..]),
        "sl" => sl::run(&args[2..]),
        _ => return,
    }
}

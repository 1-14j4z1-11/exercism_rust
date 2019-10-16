#[macro_use]
extern crate clap;
extern crate game_of_life;

use std::str::FromStr;
use std::iter::Iterator;
use std::thread::sleep;
use std::time::Duration;
use clap::{App, Arg};
use game_of_life::{GameModel, Rule};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("input_file")
            .help("Input file path")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("max_step")
            .help("Max step")
            .short("s")
            .long("step")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("rule")
            .help("Rule")
            .short("r")
            .long("rule")
            .takes_value(true));

    let args = match parse_args(app) {
        Ok(x) => x,
        Err(_) => {
            return;
        },
    };

    let path = args.0;
    let step = args.1;
    let rule = args.2;

    let content = match read_file(&path) {
        Ok(lines) => lines,
        _ => {
            println!("Could not open file : {}", path);
            return
        },
    };

    let live_char = content.0;
    let dead_char = content.1;
    let input_lines = content.2;
    let mut model = match GameModel::from_str_lines(&input_lines.iter().skip(1).map(|l| l.as_ref()).collect::<Vec<_>>(), live_char, rule) {
        Some(m) => m,
        None => {
            
            return
        },
    };

    for i in 0..step {
        let lines = model.to_str_lines(live_char, dead_char);

        println!();
        println!("[{}]", i);

        for l in lines {
            println!("{}", l);
        }

        model.next();
        sleep(Duration::from_millis(200));
    }
}

fn read_file(path: &str) -> Result<(char, char, Vec<String>), ()> {
    let lines = match std::fs::read_to_string(path) {
        Ok(text) => text.split(|c| c == '\r' || c == '\n').filter(|s| s.len() > 0).map(str::to_string).collect::<Vec<_>>(),
        Err(_) => return Err(()),
    };

    let header = match lines.iter().nth(0) {
        Some(x) => x,
        None => return Err(()),
    };

    let header_chars = header.chars().collect::<Vec<_>>();

    if header_chars.len() != 2 {
        return Err(());
    }

    Ok((header_chars[0], header_chars[1], lines.into_iter().skip(1).collect::<Vec<_>>()))
}

fn parse_args(app: App) -> Result<(String, usize, Rule), ()> {
    let matches = app.get_matches();
    let input = match matches.value_of("input_file") {
        Some(x) => x,
        None => return Err(()),
    };

    let step = match matches.value_of("max_step") {
        Some(x) => match x.parse::<usize>() {
            Ok(s) => if 1 <= s && s <= std::usize::MAX {
                s
            } else {
                return Err(());
            },
            _ => return Err(()),
        },
        None => return Err(()),
    };

    let rule = match matches.value_of("rule") {
        None => Rule::from_str("23/3").unwrap(),
        Some(x) => match Rule::from_str(x) {
            Ok(r) => r,
            _ => return Err(()),
        },
    };

    Ok((input.to_string(), step, rule))
}

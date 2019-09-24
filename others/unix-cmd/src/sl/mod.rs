use crate::console;
use std::{cmp, thread, time};

use crate::args;
mod animation_c51;
mod animation_d51;
mod animation_logo;
mod animation_smoke;

const FRAME_DELAY_MS: u64 = 50;
const FLYING_FRAME_STEP: usize = 10;

pub fn run(args: &[&str]) {
    let options = args::parse_options(args);
    let mut frame = 0;
    let fly = options.contains(&'F');
    let mut empty_line = String::new();


    let animation_info: (usize, fn(usize) -> std::vec::Vec<&'static str>) =
        if options.contains(&'l') {
            (animation_logo::FUNNEL, animation_logo::get)
        } else if options.contains(&'c') {
            (animation_c51::FUNNEL, animation_c51::get)
        } else {
            (animation_d51::FUNNEL, animation_d51::get)
        };

    loop {
        let offset_smokes = add_prefix_spaces(animation_info.0, &animation_smoke::get(frame));
        let mut lines = offset_smokes.iter().map(|x| &**x).collect::<Vec<&str>>();
        lines.append(&mut animation_info.1(frame));
        
        if empty_line.len() == 0 {
            empty_line = create_empty_line(lines.iter().map(|&l| l.len()).max().unwrap());
        }

        if fly {
            for _ in 0..(frame / FLYING_FRAME_STEP) {
                lines.remove(0);
                lines.push(empty_line.as_str());
            }
        }

        let written_lines = print_animation(frame, &lines);

        if written_lines == 0 {
            console::clear();
            return;
        }

        frame += 1;
        console::move_up(written_lines);
        thread::sleep(time::Duration::from_millis(FRAME_DELAY_MS));
        continue;
    }
}

fn print_animation(frame: usize, lines: &[&str]) -> usize {
    let console_width = console::width();
    let start_x = console_width as isize - frame as isize - 1;
    let n_spaces = cmp::min(cmp::max(start_x, 0), console_width as isize) as usize;
    let line_start = cmp::max(-start_x, 0) as usize;
    let line_end = lines.iter().map(|&l| l.len()).max().unwrap();

    if line_start >= line_end {
        return 0;
    }

    for line in lines {
        let mut s = String::new();

        for _ in 0..n_spaces {
            s.push(' ');
        }

        let end = cmp::min(line.len(), line_start + console_width - n_spaces);
        if line_start < end {
            s.push_str(&line[line_start..end]);
        }
        println!("{}", s);
    }

    lines.len()
}

fn add_prefix_spaces(n_spaces: usize, lines: &[&str]) -> Vec<String> {
    let mut new_lines = vec![];
    let space = (0..n_spaces).map(|_| ' ').collect::<String>();

    for line in lines {
        new_lines.push(format!("{}{}", space, line));
    }

    new_lines
}

fn create_empty_line(len: usize) -> String {
    let mut line = String::new();

    for _ in 0..len {
        line.push(' ');
    }

    line
}

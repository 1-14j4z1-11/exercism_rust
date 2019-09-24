use crate::args;
use crate::console;
use std::iter::Iterator;

pub fn run(args: &[&str]) {
    let options = args::parse_options(args);
    let mut files = get_files_in_current_dir();
    
    if !options.contains(&'a') {
        files.retain(|f| !f.starts_with("."));
    }

    if options.contains(&'1') {
        print_single_line(&files);
    } else if options.contains(&'m') {
        print_comma_saparated(&files);    
    } else {
        print_default(&files);
    }
}

fn print_default(files: &[String]) {
    let max_len = files.iter().map(|f| f.len()).max().unwrap();
    let n_rows = console::width() / (max_len + 1);
    let file_len = console::width() / n_rows;

    for files_1line in files.chunks(n_rows) {
        for file in files_1line {
            print!("{}", file);
            print_space(file_len - file.len());
        }
        println!();
    }
}

fn print_single_line(files: &[String]) {
    for file in files {
        println!("{}", file);
    }
}

fn print_comma_saparated(files: &[String]) {
    println!("{}", files.join(", "));
}

fn get_files_in_current_dir() -> Vec<String> {
    let dir = "./";
    let mut files = std::fs::read_dir(dir)
        .unwrap()
        .map(|p| {
            let path = p.unwrap().path();
            let suffix = if path.is_dir() {
                "/"
            } else {
                ""
            };

            format!("{}{}", path.display().to_string().replacen(dir, "", 1), suffix)
        })
        .collect::<Vec<_>>();

    files.insert(0, "../".to_string());
    files.insert(0, "./".to_string());

    files
}

fn print_space(n: usize) {
    for _ in 0..n {
        print!(" ");
    }
}

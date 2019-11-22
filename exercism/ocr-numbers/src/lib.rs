// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::iter::Iterator;

const ROW_SIZE: usize = 4;
const COLUMN_SIZE: usize = 3;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut numbers: Vec<String> = vec![];

    let size = match get_grid_size(&lines) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    for r in 0..size.0 {
        let mut decode_str = String::new();

        for c in 0..size.1 {
            let clip = clip_to_string(&lines, r, c);
            decode_str.push(decode_to_number(clip));
        }

        numbers.push(decode_str);
    }

    Ok(numbers.join(","))
}

fn get_grid_size(lines: &[&str]) -> Result<(usize, usize), Error> {
    if lines.len() % ROW_SIZE != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }

    let row = lines.len() / ROW_SIZE;
    let col_chars = lines.iter().find(|_| true).unwrap().len();

    if col_chars % COLUMN_SIZE != 0 {
        return Err(Error::InvalidColumnCount(col_chars));
    }

    if lines.iter().any(|x| x.len() != col_chars) {
        return Err(Error::InvalidColumnCount(
            lines.iter().find(|x| x.len() != col_chars).unwrap().len(),
        ));
    }

    let col = col_chars / COLUMN_SIZE;
    Ok((row, col))
}

fn clip_to_string(lines: &[&str], row: usize, col: usize) -> String {
    let mut clipped = String::new();

    for line in lines.iter().skip(row * ROW_SIZE).take(ROW_SIZE) {
        let sub = &line[(col * COLUMN_SIZE)..((col + 1) * COLUMN_SIZE)];
        clipped = format!("{}{}", clipped, sub);
    }

    clipped
}

fn decode_to_number(string: String) -> char {
    match &string as &str {
        " _ | ||_|   " => '0',
        "     |  |   " => '1',
        " _  _||_    " => '2',
        " _  _| _|   " => '3',
        "   |_|  |   " => '4',
        " _ |_  _|   " => '5',
        " _ |_ |_|   " => '6',
        " _   |  |   " => '7',
        " _ |_||_|   " => '8',
        " _ |_| _|   " => '9',
        _ => '?',
    }
}

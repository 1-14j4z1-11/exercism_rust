pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let width = match minefield.iter().map(|&s| s.len()).max() {
        Some(x) => x,
        None => 0,
    };
    let height = minefield.len();
    let mut marked_field: Vec<String> = vec![];

    for y in 0..height {
        let mut marked_row = String::new();

        for x in 0..width {
            if get_field_at(minefield, x, y).unwrap() == '*' {
                marked_row.push('*');
            } else {
                marked_row.push(count_to_char(count_mark_around(
                    minefield, x as isize, y as isize,
                )));
            }
        }

        marked_field.push(marked_row);
    }

    marked_field
}

fn count_to_char(count: u32) -> char {
    match count {
        0 => ' ',
        x => format!("{}", x % 10).chars().nth(0).unwrap(),
    }
}

fn count_mark_around(minefield: &[&str], x: isize, y: isize) -> u32 {
    let mut count = 0;

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }

            match get_field_at(minefield, (x + i) as usize, (y + j) as usize) {
                Some('*') => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn get_field_at(minefield: &[&str], x: usize, y: usize) -> Option<char> {
    match minefield.iter().nth(y) {
        Some(row) => row.chars().nth(x),
        None => None,
    }
}

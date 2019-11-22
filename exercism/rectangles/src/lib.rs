const CORNER: char = '+';
const LINE_H: char = '-';
const LINE_V: char = '|';

pub fn count(lines: &[&str]) -> u32 {
    let mut n_rects = 0;
    let mut corners = vec![];

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if get_at(lines, x, y) != CORNER {
                continue;
            }
            corners.push((x, y));
        }
    }

    for &point in &corners {
        for &other in &corners {
            if point.0 >= other.0 || point.1 >= other.1 {
                continue;
            }

            if is_rectangle(lines, point.0, point.1, other.0, other.1) {
                n_rects += 1;
            }
        }
    }

    n_rects
}

fn is_rectangle(lines: &[&str], x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    assert!(
        x1 < x2 && y1 < y2,
        "required (x1, y1) < (x2, y2) : (x1, y1) = ({}, {}), (x2, y2) = ({}, {})",
        x1,
        y1,
        x2,
        y2
    );

    if get_at(lines, x1, y1) != CORNER
        || get_at(lines, x1, y2) != CORNER
        || get_at(lines, x2, y1) != CORNER
        || get_at(lines, x2, y2) != CORNER
    {
        return false;
    }

    for i in (x1 + 1)..x2 {
        let c_xi_y1 = get_at(lines, i, y1);
        let c_xi_y2 = get_at(lines, i, y2);

        if (c_xi_y1 != CORNER && c_xi_y1 != LINE_H) || (c_xi_y2 != CORNER && c_xi_y2 != LINE_H) {
            return false;
        }
    }

    for i in (y1 + 1)..y2 {
        let c_x1_yi = get_at(lines, x1, i);
        let c_x2_yi = get_at(lines, x2, i);

        if (c_x1_yi != CORNER && c_x1_yi != LINE_V) || (c_x2_yi != CORNER && c_x2_yi != LINE_V) {
            return false;
        }
    }

    true
}

fn get_at(lines: &[&str], x: usize, y: usize) -> char {
    lines[y].chars().nth(x).unwrap()
}

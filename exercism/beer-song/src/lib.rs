pub fn verse(n: i32) -> String {
    if n > 1 {
        format!("{0} {1} of beer on the wall, {0} {1} of beer.\nTake one down and pass it around, {2} {3} of beer on the wall.\n", n, bottle_unit(n), n - 1, bottle_unit(n - 1))
    } else if n == 1 {
        format!("{0} {1} of beer on the wall, {0} {1} of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, bottle_unit(n))
    }else {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut text = String::new();

    for v in (end..(start + 1)).rev().map(|n| verse(n)) {
        text = if text == "" {
            v
        } else {
            format!("{}\n{}", text, v)
        }
    }

    text
}

fn bottle_unit(n: i32) -> &'static str {
    if n == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

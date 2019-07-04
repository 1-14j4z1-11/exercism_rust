extern crate regex;

use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    let number_regex = Regex::new(
        r"^\s*(?:\+?(1)[\.\- ]*)?\(?([2-9]\d\d)\)?[\.\- ]*([2-9]\d\d)[\.\- ]*(\d{4})\s*$",
    )
    .unwrap();

    match number_regex.captures(user_number) {
        None => None,
        Some(caps) => Some(format!(
            "{}{}{}",
            group_at(&caps, 2),
            group_at(&caps, 3),
            group_at(&caps, 4)
        )),
    }
}

fn group_at<'a>(caps: &'a regex::Captures, group: usize) -> &'a str {
    caps.get(group).unwrap().as_str()
}

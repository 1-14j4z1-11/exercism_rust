extern crate regex;

use regex::Captures;
use regex::Regex;

pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    if match_regex(r"^([aeiou]|xr|yt)", word).is_some() {
        return format!("{}ay", word);
    }

    if let Some(cap) = match_regex(r"^([^aeiou]+)(y(?:.*))$", word) {
        return format!(
            "{}{}ay",
            cap.get(2).unwrap().as_str(),
            cap.get(1).unwrap().as_str()
        );
    }

    if let Some(cap) = match_regex(r"^([^aeiou]*(?:qu)|[^aeiou]+)(.*)$", word) {
        return format!(
            "{}{}ay",
            cap.get(2).unwrap().as_str(),
            cap.get(1).unwrap().as_str()
        );
    }

    word.to_string()
}

fn match_regex<'a>(pattern: &'a str, input: &'a str) -> Option<Captures<'a>> {
    Regex::new(pattern).unwrap().captures(input)
}

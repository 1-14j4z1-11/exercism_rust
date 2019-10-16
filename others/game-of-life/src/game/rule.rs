extern crate regex;

use regex::Regex;
use std::str::FromStr;

pub struct Rule {
    keep: Vec<usize>,
    birth: Vec<usize>,
}

impl Rule {
    pub fn new(keep: &[usize], birth: &[usize]) -> Self {
        Rule {
            keep: keep.iter().map(|&x| x).collect::<Vec<_>>(),
            birth: birth.iter().map(|&x| x).collect::<Vec<_>>(),
        }
    }

    pub fn next_state(&self, current: bool, n_lives: usize) -> bool {
        if self.birth.contains(&n_lives) {
            true
        } else if self.keep.contains(&n_lives) {
            current
        } else {
            false
        }
    }
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::from_str(r"^\s*(\d+)\s*/\s*(\d+)\s*$").unwrap();
        let caps = match regex.captures(s) {
            Some(c) => c,
            None => return Err(()),
        };

        let str_keep = caps.get(1).unwrap().as_str();
        let str_birth = caps.get(2).unwrap().as_str();

        let keep = str_keep
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let birth = str_birth
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();

        Ok(Rule::new(&keep[..], &birth))
    }
}

#[test]
fn test_from_str() {
    let ok_testcase = [
        ("23/3", vec![2, 3], vec![3]),
        (" 3 / 4 ", vec![3], vec![4]),
        ("123456789/0", vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![0]),
        ("0/123456789", vec![0], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
    ];

    for (input, exp_keep, exp_birth) in ok_testcase.iter() {
        let rule = Rule::from_str(input).unwrap();
        assert_eq!(rule.keep, *exp_keep);
        assert_eq!(rule.birth, *exp_birth);
    }

    let ng_testcase = [
        "23/",
        "/1",
        "A/a",
    ];

    for input in ng_testcase.iter() {
        assert_eq!(Rule::from_str(input).is_err(), true);
    }
}

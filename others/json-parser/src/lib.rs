extern crate regex;

use regex::{Captures, Regex};
use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

pub fn parse(text: &str) -> Result<Json, ()> {
    let (s, mut replace_map) = stash_strings(text);
    let s = remove_white_spaces(&s);
    match parse_recursive(&s, &mut replace_map) {
        Ok(json) => Ok(json),
        Err(msg) => {
            println!("{}", msg);
            Err(())
        },
    }
}

fn parse_recursive(text: &str, replace_map: &mut HashMap<String, String>) -> Result<Json, String> {
    if let Some(json) = match_unwrapped_str(text) {
        return Ok(json);
    }

    if let Some(json_result) = match_stashed_str(text, replace_map) {
        return match json_result {
            Ok(json) => Ok(json),
            Err(msg) => Err(msg),
        };
    }

    if let Some(json) = match_number(text) {
        return Ok(json);
    }

    if let Some(json_result) = match_map(text, replace_map) {
        return match json_result {
            Ok(json) => Ok(json),
            Err(msg) => Err(msg),
        };
    }

    if let Some(json_result) = match_array(text, replace_map) {
        return match json_result {
            Ok(json) => Ok(json),
            Err(msg) => Err(msg),
        };
    }

    Err(format!("Unknown format"))
}

fn stash_strings(s: &str) -> (String, HashMap<String, String>) {
    let regex = Regex::from_str(r#""((\\\\)*(\\.)?([^"\\]+)?)*""#).unwrap();
    let mut index = 0;
    let mut map = HashMap::new();

    let replaced_str = regex.replace_all(s, |caps: &Captures| {
        let raw_str = caps.get(0).unwrap().as_str();
        let unwrapped_str = raw_str[1..(raw_str.len() - 1)].to_string();
        let replace = format!("${}", index);
        map.insert(replace.clone(), unwrapped_str);
        index += 1;
        replace
    }).as_ref().to_string();

    (replaced_str, map)
}

fn remove_white_spaces(s: &str) -> String {
    let regex = Regex::from_str(r"\s+").unwrap();
    regex.replace_all(s, |_: &Captures| "").as_ref().to_string()
}

fn match_number(s: &str) -> Option<Json> {
    let regex = Regex::from_str(r"^\s*([+\-]?\d+(?:\.\d+)?(?:[eE][+\-]?\d+)?)\s*$").unwrap();

    match regex.captures(s) {
        Some(cap) => {
            return Some(Json::Number(
                f64::from_str(cap.get(1).unwrap().as_str()).unwrap(),
            ));
        }
        None => None,
    }
}

fn match_unwrapped_str(s: &str) -> Option<Json> {
    return match s {
        "null" => Some(Json::Null),
        "true" => Some(Json::Boolean(true)),
        "false" => Some(Json::Boolean(false)),
        _ => None,
    };
}

fn match_stashed_str(s: &str, replace_map: &mut HashMap<String, String>) -> Option<Result<Json, String>> {
    match replace_map.remove(s) {
        Some(v) => match unescape_str(&v) {
            Some(json_str) => Some(Ok(Json::String(json_str))),
            None => Some(Err(format!("Invalid string : {}", v))),
        },
        None => None,
    }
}

fn unescape_str(s: &str) -> Option<String> {
    let chars = s.chars().collect::<Vec<_>>();
    let mut unescaped_str = String::new();
    let mut escaping = false;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        match (c, escaping) {
            ('\\', false) => {},
            ('\\', true) => unescaped_str.push('\\'),
            ('\"', true) => unescaped_str.push('\"'),
            ('/', true) => unescaped_str.push('/'),
            ('b', true) => unescaped_str.push('\x08'),
            ('f', true) => unescaped_str.push('\x0c'),
            ('n', true) => unescaped_str.push('\n'),
            ('r', true) => unescaped_str.push('\r'),
            ('t', true) => unescaped_str.push('\t'),
            ('u', true) => {
                match unescape_unicode(&chars, i - 1) {
                    None => return None,
                    Some((c, seek)) => {
                        unescaped_str.push(c);
                        i += seek - 1;
                    },
                }
            },
            (c, false) => unescaped_str.push(c),
            (_, true) => return None,
        }
        
        if c == '\\' || escaping {
            escaping = !escaping;
        }

        i += 1;
    }

    Some(unescaped_str)
}

fn unescape_unicode(chars: &[char], i: usize) -> Option<(char, usize)> {
    if chars.len() < i + 6 || chars[i] != '\\' || chars[i + 1] != 'u' {
        return None;
    }

    let u1 = match u32::from_str_radix(&chars[(i + 2)..(i + 6)].iter().collect::<String>(), 16) {
        Ok(x) => x,
        Err(_) => return None,
    };

    if u1 < 0xD800 {
        return Some((std::char::from_u32(u1).unwrap(), 6));
    } else if 0xDBFF < u1 {
        return None;
    }

    if chars.len() < i + 11 || chars[i + 6] != '\\' || chars[i + 7] != 'u' {
        return None;
    }

    let u2 = match u32::from_str_radix(&chars[(i + 8)..(i + 12)].iter().collect::<String>(), 16) {
        Ok(x) => x,
        Err(_) => return None,
    };
    
    if u2 < 0xDC00 || 0xDFFF < u2 {
        return None;
    }

    let u = 0x10000 + (u1 - 0xD800) * 0x400 + (u2 - 0xDC00);
    return Some((std::char::from_u32(u).unwrap(), 12));
}

fn match_map(s: &str, replace_map: &mut HashMap<String, String>) -> Option<Result<Json, String>> {
    let chars = s.chars().collect::<Vec<_>>();
    let end_index = chars.len() - 1;

    if chars[0] != '{' {
        return None;
    }

    match next_close_symbol(&chars, 0, '{', '}') {
        Some(x) if x == end_index => {},
        _ => return Some(Err(format!("Invalid object"))),
    }
    
    let mut obj = HashMap::new();
    let mut key_range = (None, None);
    let mut value_start = None;
    let mut depth = 0;
    
    for ptr in 1..end_index {
        match (depth, chars[ptr]) {
            (0, ':') => match key_range {
                (Some(start), _) => {
                    key_range = (Some(start), Some(ptr - 1));
                },
                _ => return Some(Err(format!("Invalid symbol => ':'"))),
            },
            (0, ',') => match (key_range, value_start) {
                ((Some(ks), Some(ke)), Some(vs)) => {
                    let key = chars[ks..=ke].iter().collect::<String>();
                    let value = chars[vs..ptr].iter().collect::<String>();
                    match (parse_recursive(&key, replace_map), parse_recursive(&value, replace_map)) {
                        (Ok(Json::String(k)), Ok(v)) => obj.insert(k, v),
                        _ => return Some(Err(format!("Invalid Key value pair : Key : Value = {} : {}", key, value))),
                    };
                    key_range = (None, None);
                    value_start = None;
                },
                _ => return Some(Err(format!("Invalid symbol : ','"))),
            },
            (_, '{') | (_, '[') => match (key_range, value_start) {
                ((None, _), _) => return Some(Err(format!("Invalid symbol : '{{'"))),
                ((Some(_), Some(_)), None) => {
                    value_start = Some(ptr);
                    depth += 1;
                },
                _ => depth += 1,
            },
            (_, '}') | (_, ']') => depth -= 1,
            _ => match (key_range, value_start) {
                ((None, _), _) => {
                    key_range = (Some(ptr), None);
                },
                ((Some(_), Some(_)), None) => {
                    value_start = Some(ptr)
                },
                _ => {},
            },
        }
    }

    match (key_range, value_start) {
        ((Some(ks), Some(ke)), Some(vs)) => {
            let key = chars[ks..=ke].iter().collect::<String>();
            let value = chars[vs..end_index].iter().collect::<String>();
            match (parse_recursive(&key, replace_map), parse_recursive(&value, replace_map)) {
                (Ok(Json::String(k)), Ok(v)) => obj.insert(k, v),
                _ => return Some(Err(format!("Invalid Key value pair : {} : {}", key, value))),
            };
        },
        ((None, _), _) if obj.len() == 0 => {},
        ((ks, ke), vs) => return Some(Err(format!("Invalid sequence end in object : K = {:?}-{:?}, V = {:?}-End", ks, ke, vs))),
    }

    Some(Ok(Json::Object(Box::new(obj))))
}

fn match_array(s: &str, replace_map: &mut HashMap<String, String>) -> Option<Result<Json, String>> {
    let chars = s.chars().collect::<Vec<_>>();
    let end_index = chars.len() - 1;

    if chars[0] != '[' {
        return None;
    }

    match next_close_symbol(&chars, 0, '[', ']') {
        Some(x) if x == end_index => {},
        _ => return Some(Err(format!("Invalid array"))),
    }
    
    let mut array = vec![];
    let mut value_start = None;
    let mut depth = 0;
    
    for ptr in 1..end_index {
        match (depth, chars[ptr]) {
            (0, ',') => match value_start {
                Some(vs) => {
                    let value = chars[vs..ptr].iter().collect::<String>();
                    match parse_recursive(&value, replace_map) {
                        Ok(v) => array.push(v),
                        _ => return Some(Err(format!("Invalid value = {} ", value))),
                    };
                    value_start = None;
                },
                _ => return Some(Err(format!("Invalid symbol : ','"))),
            },
            (_, '[') | (_, '{') => match value_start {
                None => {
                    value_start = Some(ptr);
                    depth += 1;
                },
                _ => depth += 1,
            },
            (_, ']') | (_, '}') => depth -= 1,
            _ => match value_start {
                None => value_start = Some(ptr),
                _ => {},
            },
        }
    }

    match value_start {
        Some(vs) => {
            let value = chars[vs..end_index].iter().collect::<String>();
            match parse_recursive(&value, replace_map) {
                Ok(v) => array.push(v),
                _ => return Some(Err(format!("Invalid value = {} ", value))),
            };
        },
        _ if array.len() == 0 => {},
        _ => return Some(Err("Invalid symbol ','".to_string())),
    }

    Some(Ok(Json::Array(array)))
}

fn next_close_symbol(code: &[char], pos_open: usize, open: char, close: char) -> Option<usize> {
    if code[pos_open] != open {
        panic!();
    }

    let mut inners = 0;

    for (ptr, op) in code.iter().enumerate().skip(pos_open + 1) {
        match op {
            x if *x == open => inners += 1,
            x if *x == close => {
                if inners == 0 {
                    return Some(ptr);
                } else {
                    inners -= 1;
                }
            }
            _ => {}
        }
    }

    None
}

#[test]
fn test_single_content() {
    let testcases = vec![
        ("null", Json::Null),
        ("true", Json::Boolean(true)),
        ("false", Json::Boolean(false)),
        ("0", Json::Number(0.0)),
        ("+10", Json::Number(10.0)),
        ("-1.25", Json::Number(-1.25)),
        ("12.125", Json::Number(12.125)),
        ("2e-8", Json::Number(2e-8)),
        ("-1.2E+8", Json::Number(-1.2e8)),
        ("+24.675e8", Json::Number(24.675e8)),
        (r#""""#, Json::String(r#""#.to_string())),
        (r#""A""#, Json::String("A".to_string())),
        (r#""string""#, Json::String("string".to_string())),
        (
            r#""A,B,C,{,},[,]""#,
            Json::String("A,B,C,{,},[,]".to_string()),
        ),
        (r#""\\""#, Json::String(r"\".to_string())),
        (r#""\/""#, Json::String("/".to_string())),
        (r#""\"""#, Json::String(r#"""#.to_string())),
        (r#""\r""#, Json::String("\r".to_string())),
        (r#""\n""#, Json::String("\n".to_string())),
        (r#""\b""#, Json::String("\x08".to_string())),
        (r#""\f""#, Json::String("\x0c".to_string())),
        (r#""\t""#, Json::String("\t".to_string())),
        (r#""\u004f""#, Json::String("\x4f".to_string())),
        (r#""\uD800\uDC00""#, Json::String("\u{10000}".to_string())),
        (r#""\udbff\udfff""#, Json::String("\u{10FFFF}".to_string())),
        (r#""\\uDC00""#, Json::String("\\uDC00".to_string())),
        (r#""\\uDBFF\\uE000""#, Json::String("\\uDBFF\\uE000".to_string())),
        (r#""\\uDBFF\\uDBFF""#, Json::String("\\uDBFF\\uDBFF".to_string())),
        (r#""\\u1234""#, Json::String("\\u1234".to_string())),
        (
            r#""あいうえお""#,
            Json::String("あいうえお".to_string()),
        ),
        (r#""𪗱𪘚""#, Json::String("𪗱𪘚".to_string())),
        (r#""\uD867\uDE3D""#, Json::String("𩸽".to_string())),
        (r#""\\\\""#, Json::String(r"\\".to_string())),
        (r#""\\\\\"""#, Json::String(r#"\\""#.to_string())),
        (r#""\\\r""#, Json::String("\\\r".to_string())),
        (
            r#""\\\\\n\\\t\\\f""#,
            Json::String("\\\\\n\\\t\\\x0c".to_string()),
        ),
        (r#""\\\\\\\t""#, Json::String("\\\\\\\t".to_string())),
        (
            r#""\\\\\\\\\u1FFF""#,
            Json::String("\\\\\\\\\u{1FFF}".to_string()),
        ),
        (r#""\\\\r""#, Json::String("\\\\r".to_string())),
        (
            r#""\\\\\\n\\\\f\\""#,
            Json::String("\\\\\\n\\\\f\\".to_string()),
        ),
        (r#""\\\\\\\\t""#, Json::String("\\\\\\\\t".to_string())),
        (r#""\\\\\\v""#, Json::String("\\\\\\v".to_string())),
    ];

    for (input, exp) in &testcases {
        let result = parse(input);
        assert_eq!(result.is_ok(), true, "Input = {}", input);
        assert_eq!(result.unwrap(), *exp, "Input = {}", input);

        let input = format!(" \t\r\n{} \t\r\n", input);
        let result = parse(&input);
        assert_eq!(result.is_ok(), true, "Input = {}", input);
        assert_eq!(result.unwrap(), *exp, "Input = {}", input);
    }
}

#[test]
fn test_invalid_single_content() {
    let testcases = [
        "Null",
        "True",
        "False",
        "10.",
        "-1.25.0",
        "+-12.125",
        "-12.125e10.8",
        "12.125E10.",
        r#"A"""#,
        r#"""A"#,
        r#"A"""#,
        r#"""""#,
        r#""\""#,
        r#""\\"""#,
        r#""\"\\"\""#,
        r#""\\\\\\\v""#,
        r#""\uD800""#,
        r#""\uDC00""#,
        r#""\uDBFF\uE000""#,
        r#""\uDBFF\uDBFF""#,
    ];

    for input in &testcases {
        assert_eq!(parse(input).is_err(), true, "'{}' is invalid, but parse() returns ok.", input);
    }
}

macro_rules! map {
    ( $( $t:expr),* ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($t.0, $t.1);
            )*
            temp_map
        }
    };
}

#[test]
fn test_object() {
    let testcases = vec![
        ("{}", Json::Object(Box::new(HashMap::new()))),
        (
            r#"{ "key" : "value" }"#,
            Json::Object(Box::new(map![(
                "key".to_string(),
                Json::String("value".to_string())
            )])),
        ),
        (
            r#"{ "string":"text", "boolean":true, "number":1.5, "null":null }"#,
            Json::Object(Box::new(map![
                ("string".to_string(), Json::String("text".to_string())),
                ("boolean".to_string(), Json::Boolean(true)),
                ("number".to_string(), Json::Number(1.5)),
                ("null".to_string(), Json::Null)
            ])),
        ),
        (
            r#"{ "1": { "2" : { "3" : true }, "k2" : -0.1 }, "k1" : "string" }"#,
            Json::Object(Box::new(map![
                ("1".to_string(), Json::Object(Box::new(map![
                    ("2".to_string(), Json::Object(Box::new(map![
                        ("3".to_string(), Json::Boolean(true))
                    ]))),
                    ("k2".to_string(), Json::Number(-0.1))
                ]))),
                ("k1".to_string(), Json::String("string".to_string()))
            ])),
        ),
        (
            r#"{ "1": { "2" : { "3" : true }, "k2" : [ "V1", "V2" ] }, "k1" : "string" }"#,
            Json::Object(Box::new(map![
                ("1".to_string(), Json::Object(Box::new(map![
                    ("2".to_string(), Json::Object(Box::new(map![
                        ("3".to_string(), Json::Boolean(true))
                    ]))),
                    ("k2".to_string(), Json::Array(
                        vec![Json::String("V1".to_string()),
                            Json::String("V2".to_string())]))
                ]))),
                ("k1".to_string(), Json::String("string".to_string()))
            ])),
        ),
    ];

    for (input, exp) in &testcases {
        let result = parse(input);
        assert_eq!(result.is_ok(), true, "Input = {}", input);
        assert_eq!(result.unwrap(), *exp, "Input = {}", input);
    }
}

#[test]
fn test_invalid_object() {
    let testcases = [
        r#"{ "key" : "value" "#,
        r#" "key" : "value" }"#,
        r#"{ "key"  "value" }"#,
        r#"{ "key" , "value" }"#,
        r#"{ "key" : {"value"} }"#,
        r#"{ "key1" : "value1" }, { "key2" : "value2" }"#,
        r#"{ "key1" : "value1", "key2" : "value2", }"#,
    ];

    for input in &testcases {
        assert_eq!(parse(input).is_err(), true, "'{}' is invalid, but parse() returns ok.", input);
    }
}

#[test]
fn test_array() {
    let testcases = vec![
        ("[]", Json::Array(vec![])),
        (
            r#"["A", "B", "C"]"#,
            Json::Array(vec![
                Json::String("A".to_string()),
                Json::String("B".to_string()),
                Json::String("C".to_string()),
            ]),
        ),
        (
            r#"["A", "B", [ "C1", "C2", "C3", ["D1", "D2"] ], ["E"], []]"#,
            Json::Array(vec![
                Json::String("A".to_string()),
                Json::String("B".to_string()),
                Json::Array(vec![
                    Json::String("C1".to_string()),
                    Json::String("C2".to_string()),
                    Json::String("C3".to_string()),
                    Json::Array(vec![
                        Json::String("D1".to_string()),
                        Json::String("D2".to_string()),
                    ]),
                ]),
                Json::Array(vec![
                    Json::String("E".to_string()),
                ]),
                Json::Array(vec![]),
            ]),
        ),
        (
            r#"["A", "B", [ "C1", "C2", "C3", {"D1" : "D2"}, {"E1" : "E2"} ], ["E"], []]"#,
            Json::Array(vec![
                Json::String("A".to_string()),
                Json::String("B".to_string()),
                Json::Array(vec![
                    Json::String("C1".to_string()),
                    Json::String("C2".to_string()),
                    Json::String("C3".to_string()),
                    Json::Object(Box::new(map![
                        ("D1".to_string(), Json::String("D2".to_string()))
                    ])),
                    Json::Object(Box::new(map![
                        ("E1".to_string(), Json::String("E2".to_string()))
                    ])),
                ]),
                Json::Array(vec![
                    Json::String("E".to_string()),
                ]),
                Json::Array(vec![]),
            ]),
        ),
    ];

    for (input, exp) in &testcases {
        let result = parse(input);
        assert_eq!(result.is_ok(), true, "Input = {}", input);
        assert_eq!(result.unwrap(), *exp, "Input = {}", input);
    }
}

#[test]
fn test_invalid_array() {
    let testcases = [
        r#"[ value" "#,
        r#" value" ]"#,
        r#"[ "value" ]]"#,
        r#"[ "key" : "value" ]"#,
        r#"[ "value1" ], [ value2" ]"#,
        r#"[ "value1", "value2", ]"#,
    ];

    for input in &testcases {
        assert_eq!(parse(input).is_err(), true, "'{}' is invalid, but parse() returns ok.", input);
    }
}

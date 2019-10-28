extern crate regex;

use std::collections::HashMap;
use std::str::FromStr;
use regex::{Captures, Regex};

#[derive(PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

pub fn parse(text: &str) -> Result<Json, ()> {

    if let Some(json) = match_number(text) {
        return Ok(json);
    }

    if let Some(json) = match_unwrapped_str(text) {
        return Ok(json);
    }

    if let Some(json_result) = match_wrapped_str(text) {
        return match json_result {
            Ok(json) => Ok(json),
            Err(_) => Err(()),
        };
    }

    Err(())
}

fn match_number(s: &str) -> Option<Json> {
    let regex_number = Regex::from_str(r"^\s*([+\-]?\d+(?:\.\d+)?(?:[eE][+\-]?\d+)?)\s*$").unwrap();

    match regex_number.captures(s) {
        Some(cap) => {
            return Some(Json::Number(f64::from_str(cap.get(1).unwrap().as_str()).unwrap()));
        },
        None => None,
    }
}

fn match_unwrapped_str(s: &str) -> Option<Json> {
    let regex_unwrapped_str = Regex::from_str(r#"^\s*(null|true|false)\s*$"#).unwrap();

    match regex_unwrapped_str.captures(s) {
        Some(cap) => {
            return match cap.get(1).unwrap().as_str() {
                "null" => Some(Json::Null),
                "true" => Some(Json::Boolean(true)),
                "false" => Some(Json::Boolean(false)),
                _ => None,
            };
        },
        None => None,
    }
}

fn match_wrapped_str(s: &str) -> Option<Result<Json, ()>> {
    let regex_wrapped_str = Regex::from_str(r#"^\s*"(.*)"\s*$"#).unwrap();

    match regex_wrapped_str.captures(s) {
        Some(cap) => {
            match unescape_str(cap.get(1).unwrap().as_str()) {
                Some(e) => return Some(Ok(Json::String(e))),
                None => return Some(Err(())),
            }
        },
        None => None,
    }
}

fn unescape_str(s: &str) -> Option<String> {
    let escape_pattern = r#"u[0-9a-fA-F]{4}|."#;
    let regex_escape = Regex::from_str(&format!(r"\\((?:\\\\)*)({0})", escape_pattern)).unwrap();
    
    let escaped = regex_escape.replace_all(s, |caps: &Captures| {
        let symbol = caps.get(2).unwrap().as_str();

        if let Some(cap_bs) = caps.get(1) {
            let bs = cap_bs.as_str();
            bs[0..(bs.len() / 2)].to_string() + &unescape_symbol(symbol)
        } else {
            unescape_symbol(symbol)
        }
    }).to_string();

    Some(escaped)
}

fn unescape_symbol(s: &str) -> String {
    match s {
        "\\" => "\\".to_string(),
        "\"" => "\"".to_string(),
        "/" => "/".to_string(),
        "b" => "\x08".to_string(),
        "f" => "\x0c".to_string(),
        "n" => "\n".to_string(),
        "r" => "\r".to_string(),
        "t" => "\t".to_string(),
        x if x.starts_with("u") => {
            match std::char::from_u32(u32::from_str_radix(&x[1..], 16).unwrap()) {
                Some(c) => format!("{}", c),
                None => x.to_string(),
            }
        },
        x => x.to_string(),
    }
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
        (r#""A,B,C,{,},[,]""#, Json::String("A,B,C,{,},[,]".to_string())),
        (r#""\\""#, Json::String(r"\".to_string())),
        (r#""\/""#, Json::String("/".to_string())),
        (r#""\"""#, Json::String(r#"""#.to_string())),
        (r#""\r""#, Json::String("\r".to_string())),
        (r#""\n""#, Json::String("\n".to_string())),
        (r#""\b""#, Json::String("\x08".to_string())),
        (r#""\f""#, Json::String("\x0c".to_string())),
        (r#""\t""#, Json::String("\t".to_string())),
        (r#""\u004f""#, Json::String("\x4f".to_string())),
        
        (r#""\\\\""#, Json::String(r"\\".to_string())),
        (r#""\\\\\"""#, Json::String(r#"\\""#.to_string())),
        (r#""\\\r""#, Json::String("\\\r".to_string())),
        (r#""\\\\\n\\\f\\""#, Json::String("\\\\\n\\\x0c\\".to_string())),
        (r#""\\\\\\\t""#, Json::String("\\\\\\\t".to_string())),
        (r#""\\\\\\\\\u1FFF""#, Json::String("\\\\\\\\\u{1FFF}".to_string())),
        
        (r#""\\\\r""#, Json::String("\\\\r".to_string())),
        (r#""\\\\\\n\\\\f\\""#, Json::String("\\\\\\n\\\\f\\".to_string())),
        (r#""\\\\\\\\t""#, Json::String("\\\\\\\\t".to_string())),

        (r#""\\\\\\\v""#, Json::String("\\\\\\v".to_string())),
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
    let testcases  = [
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
    ];

    for input in &testcases {
        assert_eq!(parse(input).is_err(), true);
    }
}
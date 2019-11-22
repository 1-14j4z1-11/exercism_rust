pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return "".to_string();
    }

    let mut encoded = String::new();
    let mut ch = source.chars().find(|_| true).unwrap();
    let mut count = 1;

    fn append_run_length(s: String, c: char, len: usize) -> String {
        if len == 1 {
            format!("{}{}", s, c)
        } else {
            format!("{}{}{}", s, len, c)
        }
    };

    for c in source.chars().skip(1) {
        if c != ch {
            encoded = append_run_length(encoded, ch, count);
            ch = c;
            count = 1;
        } else {
            count += 1;
        }
    }

    append_run_length(encoded, ch, count)
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return "".to_string();
    }

    let mut decoded = String::new();
    let mut len_str = String::new();

    for c in source.chars() {
        if c.is_numeric() {
            len_str.push(c);
        } else {
            let len = if len_str.is_empty() {
                1
            } else {
                len_str.parse::<usize>().unwrap()
            };
            decoded = format!("{}{}", decoded, (0..len).map(|_| c).collect::<String>());
            len_str = String::new();
        }
    }

    decoded
}

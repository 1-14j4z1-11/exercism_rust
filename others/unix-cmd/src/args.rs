pub fn parse_options(args: &[&str]) -> Vec<char> {
    let mut options = vec![];

    for arg in args {
        if !arg.starts_with("-") {
            continue;
        }

        for c in arg.chars().skip(1) {
            options.push(c);
        }
    }

    options
}

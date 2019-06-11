pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut text = String::new();

    for i in 0..(list.len() - 1) {
        text = format!(
            "{}For want of a {} the {} was lost.\n",
            text,
            list[i],
            list[i + 1]
        );
    }

    format!(
        "{}And all for the want of a {}.",
        text,
        match list.first() {
            Some(v) => v,
            None => "",
        }
    )
}

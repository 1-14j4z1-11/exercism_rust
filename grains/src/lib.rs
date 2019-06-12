pub fn square(s: u32) -> u64 {
    assert!(0 < s && s <= 64, "Square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..65)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|n| square(n))
        .sum()
}

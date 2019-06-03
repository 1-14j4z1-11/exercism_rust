pub fn is_leap_year(year: u64) -> bool {
    return divisible(year, 400) || !divisible(year, 100) && divisible(year, 4);
}

fn divisible(value: u64, divisor: u64) -> bool {
    return value % divisor == 0;
}

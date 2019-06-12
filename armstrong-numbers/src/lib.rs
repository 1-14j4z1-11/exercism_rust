pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = vec![];
    let mut mut_num = num;

    while mut_num != 0 {
        digits.push(mut_num % 10);
        mut_num /= 10;
    }

    let n_digits = digits.len() as u32;
    let sum: u32 = digits.iter().map(|x| x.pow(n_digits)).sum();
    sum == num
}

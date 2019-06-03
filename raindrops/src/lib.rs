use std::vec::Vec;

pub fn raindrops(n: u32) -> String {
    let divisors = divisors_of(n);
    let mut text = String::new();

    println!("{:?}", divisors);

    if divisors.contains(&3) {
        text = format!("{}{}", text, "Pling");
    }

    if divisors.contains(&5) {
        text = format!("{}{}", text, "Plang");
    }

    if divisors.contains(&7) {
        text = format!("{}{}", text, "Plong");
    }

    if text == "" {
        text = format!("{}", n);
    }
    
    text
}

fn divisors_of(n: u32) -> Vec<u32> {
    let mut divisors: Vec<u32> = Vec::new();
    let end = (n as f64).sqrt() as u32;

    for x in 1..(end + 1) {
        if !divisible(n, x) {
            continue;
        }

        divisors.push(x);

        if !divisors.contains(&(n / x)) {
            divisors.push(n / x);
        }
    }

    divisors
}

fn divisible(n: u32, divisor: u32) -> bool {
    n % divisor == 0
}

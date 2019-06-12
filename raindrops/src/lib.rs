use std::vec::Vec;

pub fn raindrops(n: u32) -> String {
    let factors = factors_of(n);
    let mut text = String::new();

    println!("{:?}", factors);

    if factors.contains(&3) {
        text = format!("{}{}", text, "Pling");
    }

    if factors.contains(&5) {
        text = format!("{}{}", text, "Plang");
    }

    if factors.contains(&7) {
        text = format!("{}{}", text, "Plong");
    }

    if text == "" {
        text = format!("{}", n);
    }
    
    text
}

fn factors_of(n: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = Vec::new();
    let end = (n as f64).sqrt() as u32;

    for x in 1..(end + 1) {
        if !divisible(n, x) {
            continue;
        }

        factors.push(x);

        if !factors.contains(&(n / x)) {
            factors.push(n / x);
        }
    }

    factors
}

fn divisible(n: u32, divisor: u32) -> bool {
    n % divisor == 0
}

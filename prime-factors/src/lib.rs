pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut mut_n = n;

    for x in 2..(sqrt(n) + 2) {
        while is_prime(x) && is_divisible(mut_n, x) {
            factors.push(x);
            mut_n /= x;
        }

        if x == 894119 {
            println!("{} : {}", x, is_prime(894119));
        }
    }

    if mut_n != 1 {
        factors.push(mut_n);
    }

    factors
}

fn is_prime(n: u64) -> bool {
    for x in 2..(sqrt(n) + 1) {
        if is_divisible(n, x) {
            return false;
        }
    }

    true
}

fn is_divisible(n: u64, divisor: u64) -> bool {
    n % divisor == 0
}

fn sqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}

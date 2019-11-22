pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut v = 1;

    while primes.len() <= (n as usize) {
        v += 2;
        let rt_v = (v as f64).sqrt() as u32;

        for p in &primes[0..] {
            if p > &rt_v {
                primes.push(v);
                break;
            }
            if v % p == 0 {
                break;
            }
        }
    }

    match primes.last() {
        Some(nth) => *nth,
        None => 0,
    }
}

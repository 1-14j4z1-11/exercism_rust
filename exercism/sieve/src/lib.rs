use std::iter::Iterator;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = (2..upper_bound + 1).collect::<Vec<u64>>();
    let mut n = match primes.first() {
        Some(&x) => x,
        None => return vec![],
    };

    loop {
        primes.retain(|&x| x == n || x % n != 0);

        n = match primes.iter().filter(|&&x| x > n).nth(0) {
            Some(&x) => x,
            None => break,
        }
    }

    primes
}

use std::iter::Iterator;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut is_prime = (0..upper_bound + 1).map(|_| true).collect::<Vec<_>>();
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..is_prime.len() {
        if !is_prime[i] {
            continue;
        }

        for j in (i + 1)..is_prime.len() {
            if j % i == 0 {
                is_prime[j] = false;
            }
        }
    }

    let mut primes: Vec<u64> = Vec::new();

    for i in 1..is_prime.len() {
        if is_prime[i] {
            primes.push(i as u64);
        }
    }

    primes
}

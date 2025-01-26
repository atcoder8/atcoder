// unfinished

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        nn: [usize; t],
    }

    for &n in &nn {
        let (a, m) = solve(n);
        println!("{} {}", a, m);
    }
}

#[allow(unused)]
fn solve(n: usize) -> (usize, usize) {
    let factors = prime_factorization(n);
    let primes = factors.iter().map(|&(p, _)| p).collect_vec();

    let mut subset_mul = vec![1_usize; 1 << primes.len()];
    for bits in 1_usize..1 << primes.len() {
        let lsb = bits & bits.wrapping_neg();
        subset_mul[bits] = subset_mul[bits ^ lsb] * primes[lsb.trailing_zeros() as usize];
    }

    let mut inclusive = 0_usize;
    let mut exclusive = 0_usize;
    for bit in 0_usize..1 << primes.len() {
        if bit.count_ones() % 2 == 1 {
            inclusive += n / subset_mul[bit];
        } else {
            exclusive += n / subset_mul[bit];
        }
    }

    todo!()
}

/// Performs prime factorization of `n`.
///
/// The result of the prime factorization is returned as a
/// list of prime factor and exponent pairs.
pub fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut factors = vec![];
    let mut t = n;

    for p in 2.. {
        if p * p > t {
            break;
        }

        let mut e = 0;
        while t % p == 0 {
            t /= p;
            e += 1;
        }

        if e != 0 {
            factors.push((p, e));
        }
    }

    if t != 1 {
        factors.push((t, 1));
    }

    factors
}

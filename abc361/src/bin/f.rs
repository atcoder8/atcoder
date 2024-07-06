use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut inclusion = 0;
    let mut exclusion = 0;

    for exp in 2..=60 {
        let factors = prime_factorization(exp as usize);

        if factors.iter().any(|v| v.1 != 1) {
            continue;
        }

        let cnt = n.nth_root(exp) - 1;
        if factors.len() % 2 == 1 {
            inclusion += cnt;
        } else {
            exclusion += cnt;
        }
    }

    println!("{}", inclusion - exclusion + 1);
}

/// Performs prime factorization of `n`.
///
/// The result of the prime factorization is returned as a
/// list of prime factor and exponent pairs.
///
/// # Examples
///
/// ```
/// assert_eq!(prime_factorization(1), vec![]);
/// assert_eq!(prime_factorization(12), vec![(2, 2), (3, 1)]);
/// assert_eq!(prime_factorization(19), vec![(19, 1)]);
/// assert_eq!(prime_factorization(27), vec![(3, 3)]);
/// ```
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

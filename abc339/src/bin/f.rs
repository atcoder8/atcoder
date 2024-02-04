use std::{ops::Rem, str::FromStr};

use hashbag::HashBag;
use itertools::{iproduct, izip, Itertools};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use proconio::input;
use rand::seq::SliceRandom;

const LOWER_BOUND_MODULO: usize = 10_usize.pow(9);
const PRIME_POOL_SIZE: usize = 100;
const MODULI_NUM: usize = 20;

fn main() {
    input! {
        n: usize,
        aa: [String; n],
    }

    let mut rng = rand::thread_rng();

    let mut primes = vec![];
    primes.reserve(PRIME_POOL_SIZE);
    for cand in LOWER_BOUND_MODULO.. {
        if primality_test(cand) {
            primes.push(cand);

            if primes.len() == PRIME_POOL_SIZE {
                break;
            }
        }
    }

    let moduli = primes
        .choose_multiple(&mut rng, MODULI_NUM)
        .cloned()
        .collect_vec();

    let calc_rem = |a: &str, modulus: usize| {
        BigUint::from_str(a)
            .unwrap()
            .rem(modulus)
            .to_usize()
            .unwrap()
    };

    let create_rem_vec = |a: &str| {
        moduli
            .iter()
            .map(|&modulus| calc_rem(a, modulus))
            .collect_vec()
    };

    let rems_each_a = aa.iter().map(|a| create_rem_vec(a)).collect_vec();
    let rems_counter: HashBag<Vec<usize>> = rems_each_a.iter().cloned().collect();

    let mut ans = 0;
    for (rems1, rems2) in iproduct!(&rems_each_a, &rems_each_a) {
        let mul_rems = izip!(rems1, rems2, &moduli)
            .map(|(rem1, rem2, modulus)| rem1 * rem2 % modulus)
            .collect_vec();
        ans += rems_counter.contains(&mul_rems);
    }

    println!("{}", ans);
}

/// Returns `true` if and only if `n` is prime.
pub fn primality_test(n: usize) -> bool {
    n >= 2 && (2_usize..).take_while(|&i| i <= n / i).all(|i| n % i != 0)
}

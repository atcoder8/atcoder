use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, mut k): (usize, usize),
        aa: [usize; n],
    }

    let mut init = 0_usize;
    if 2 * k > n {
        init = aa.iter().fold(0_usize, |acc, &a| acc ^ a);
        k = n - k;
    }

    let ans = (0..n)
        .combinations(k)
        .map(|positions| positions.iter().fold(init, |xor, &pos| xor ^ aa[pos]))
        .max()
        .unwrap();
    println!("{}", ans);
}

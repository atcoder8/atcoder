use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [i64; n],
    }

    let mut cnt = 0;
    while aa.iter().filter(|&&a| a > 0).count() > 1 {
        aa.sort_unstable_by_key(|&a| Reverse(a));
        aa[0] -= 1;
        aa[1] -= 1;

        cnt += 1;
    }

    println!("{}", cnt);
}

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }

    s.sort_unstable();

    let mut ans = 0;
    for i in (0..).take_while(|&i| i * i < 10_usize.pow(n as u32)) {
        let mut digits = i.pow(2).to_string().chars().collect_vec();
        digits.resize(n, '0');
        digits.sort_unstable();

        ans += (digits == s) as usize;
    }

    println!("{}", ans);
}

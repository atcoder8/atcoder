use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let num_ones = s.iter().filter(|&&c| c == '1').count();
    let mid = enumerate(&s)
        .filter(|&(_, &c)| c == '1')
        .nth(num_ones / 2)
        .unwrap()
        .0;

    let mut ans = 0_usize;
    {
        let mut count_zeros = 0_usize;
        for &c in s[..=mid].iter().rev() {
            if c == '0' {
                count_zeros += 1;
            } else {
                ans += count_zeros;
            }
        }
    }
    {
        let mut count_zeros = 0_usize;
        for &c in s[mid + 1..].iter() {
            if c == '0' {
                count_zeros += 1;
            } else {
                ans += count_zeros;
            }
        }
    }

    println!("{}", ans);
}

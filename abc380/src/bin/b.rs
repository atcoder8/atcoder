use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut aa = vec![];
    let mut cnt = 0_usize;
    for c in s.chars().skip(1) {
        if c == '-' {
            cnt += 1;
        } else {
            aa.push(cnt);
            cnt = 0;
        }
    }

    println!("{}", aa.iter().join(" "));
}

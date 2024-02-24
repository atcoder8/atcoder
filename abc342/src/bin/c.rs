use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
        q: usize,
        cd: [(char, char); q],
    }

    let mut convert = (0..26).collect_vec();
    for &(c, d) in &cd {
        let from = c2u(c);
        let to = c2u(d);

        for x in convert.iter_mut() {
            if *x == from {
                *x = to;
            }
        }
    }

    let ans = s.chars().map(|c| u2c(convert[c2u(c)])).collect::<String>();
    println!("{}", ans);
}

fn c2u(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn u2c(u: usize) -> char {
    (u as u8 + b'a') as char
}

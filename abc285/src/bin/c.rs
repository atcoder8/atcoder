use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0_usize;

    for &c in &s {
        ans = 26 * ans + (c as u8 - b'A' + 1) as usize;
    }

    println!("{}", ans);
}

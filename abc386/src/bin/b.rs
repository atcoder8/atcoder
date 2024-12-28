use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0_usize;
    let mut idx = 0;
    while idx < s.len() {
        let double = idx + 1 < s.len() && s[idx..idx + 2] == ['0'; 2];
        let progress = 1 + double as usize;
        ans += 1;
        idx += progress;
    }
    println!("{}", ans);
}

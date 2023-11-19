use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans = 0_usize;
    let mut cnt = 0;
    for &c in &s {
        if c == '>' {
            cnt += 1;
            ans += cnt;
        } else {
            cnt = 0;
        }
    }

    println!("{}", ans);
}

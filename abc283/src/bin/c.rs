use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut ans = 0;
    let mut idx = 0;

    while idx < n {
        if idx < n - 1 && s[idx] == '0' && s[idx + 1] == '0' {
            idx += 2;
        } else {
            idx += 1;
        }

        ans += 1;
    }

    println!("{}", ans);
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut ans = 0;
    for l in 0..n {
        for r in (l + 1)..=n {
            let t = s[l..r].to_owned();
            if (0..t.len()).all(|i| t[i] == t[t.len() - 1 - i]) {
                ans = ans.max(t.len());
            }
        }
    }

    println!("{}", ans);
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut t = s.clone();
    t.sort_unstable();

    let diff = if t[0] != t[1] { t[0] } else { t[t.len() - 1] };
    let ans = s.iter().position(|&c| c == diff).unwrap() + 1;
    println!("{}", ans);
}

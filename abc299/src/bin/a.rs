use proconio::{input, marker::Chars};

fn main() {
    let ans = solve().is_some();
    println!("{}", if ans { "in" } else { "out" });
}

fn solve() -> Option<()> {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut iter = s.iter();
    iter.find(|&&c| c == '|')?;
    iter.find(|&&c| c == '*')?;
    iter.find(|&&c| c == '|')?;

    Some(())
}

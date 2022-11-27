use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = s.iter().filter(|&&c| c == 'v').count() + 2 * s.iter().filter(|&&c| c == 'w').count();
    println!("{}", ans);
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }

    let ans = s.iter().zip(&t).all(|(&x, &y)| {
        x == y || (x == '1' && y == 'l') || (x == 'l' && y == '1') || (x == '0' && y == 'o') || (x == 'o' && y == '0')
    });
    println!("{}", if ans { "Yes" } else { "No" });
}

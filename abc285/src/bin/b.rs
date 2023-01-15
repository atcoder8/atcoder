use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 1..n {
        let max_l = (0..=(n - i))
            .rev()
            .find(|&l| (0..l).all(|k| s[k] != s[k + i]))
            .unwrap();
        println!("{}", max_l);
    }
}

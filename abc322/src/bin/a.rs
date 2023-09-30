use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let ans = (0..=(n - 3)).find(|&i| s[i..(i + 3)] == vec!['A', 'B', 'C']);
    match ans {
        Some(ans) => println!("{}", ans + 1),
        None => println!("-1"),
    }
}

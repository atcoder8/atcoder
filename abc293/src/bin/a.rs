use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    for i in 1..=(s.len() / 2) {
        s.swap(2 * i - 2, 2 * i - 1);
    }

    let ans: String = s.iter().collect();
    println!("{}", ans);
}

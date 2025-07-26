use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (_n, l, r): (usize, Usize1, usize),
        s: String,
    }

    let ans = s[l..r].chars().all(|ch| ch == 'o');
    println!("{}", if ans { "Yes" } else { "No" });
}

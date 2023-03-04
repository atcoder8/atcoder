use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        pp: [Usize1; n],
    }

    let ans = pp.iter().enumerate().skip(k - 1).all(|(i, &p)| p == i);
    println!("{}", if ans { "Yes" } else { "No" });
}

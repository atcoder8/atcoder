use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    }

    let ans = n.windows(2).all(|window| window[0] > window[1]);
    println!("{}", if ans { "Yes" } else { "No" });
}

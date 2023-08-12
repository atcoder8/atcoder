use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (l, r): (Usize1, usize),
    }

    let chars = "atcoder";
    println!("{}", &chars[l..r]);
}

use itertools::izip;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        t: String,
        a: String,
    }

    let ans = izip!(t.chars(), a.chars()).any(|(v1, v2)| v1 == 'o' && v2 == 'o');
    println!("{}", if ans { "Yes" } else { "No" });
}

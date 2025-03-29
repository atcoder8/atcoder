use itertools::izip;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
        t: String,
    }

    let ans = izip!(s.chars(), t.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();
    println!("{}", ans);
}

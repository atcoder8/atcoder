use itertools::izip;
use proconio::input;

fn main() {
    input! {
        t: String,
        u: String,
    }

    let ans = (0..=t.len() - u.len()).any(|shift| {
        izip!(t[shift..].chars(), u.chars()).all(|(ch1, ch2)| ch1 == '?' || ch1 == ch2)
    });
    println!("{}", if ans { "Yes" } else { "No" });
}

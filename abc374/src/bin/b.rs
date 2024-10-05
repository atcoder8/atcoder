use itertools::izip;
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let pos = izip!(s.chars(), t.chars()).position(|(c1, c2)| c1 != c2);
    let ans = match pos {
        Some(pos) => pos + 1,
        None => {
            if s.len() == t.len() {
                0
            } else {
                s.len().min(t.len()) + 1
            }
        }
    };
    println!("{}", ans);
}

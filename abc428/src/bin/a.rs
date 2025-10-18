use num::Integer;
use proconio::input;

fn main() {
    input! {
        (s, a, b, x): (usize, usize, usize, usize),
    }

    let (q, r) = x.div_mod_floor(&(a + b));
    let dist = s * (q * a + r.min(a));
    println!("{dist}");
}

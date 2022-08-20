use proconio::input;

fn main() {
    input! {
        (s, t, x): (i64, i64, i64),
    }

    if s < t {
        println!("{}", if s <= x && x < t { "Yes" } else { "No" });
    } else {
        println!("{}", if x < t || s <= x { "Yes" } else { "No" });
    }
}

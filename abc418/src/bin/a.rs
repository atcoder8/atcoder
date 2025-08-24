use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    println!("{}", if s.ends_with("tea") { "Yes" } else { "No" });
}

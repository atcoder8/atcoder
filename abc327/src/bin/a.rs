use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s.contains("ab") || s.contains("ba");
    println!("{}", if ans { "Yes" } else { "No" });
}

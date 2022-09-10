use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{}", if t.starts_with(&s) { "Yes" } else { "No" });
}

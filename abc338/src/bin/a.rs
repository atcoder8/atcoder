use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let ans = Regex::new(r"^[A-Z][a-z]*$").unwrap().is_match(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}

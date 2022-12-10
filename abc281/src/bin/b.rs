use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"^[A-Z][1-9][0-9]{5}[A-Z]$").unwrap();
    println!("{}", if re.is_match(&s) { "Yes" } else { "No" });
}

use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"R.?M").unwrap();
    println!("{}", if re.is_match(&s) { "Yes" } else { "No" });
}

use proconio::input;
use regex::Regex;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let re = Regex::new(r"^1*/2*$").unwrap();
    let ans = n % 2 == 1 && s.chars().nth(n / 2).unwrap() == '/' && re.is_match(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}

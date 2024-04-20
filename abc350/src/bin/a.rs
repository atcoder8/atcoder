use proconio::input;
use regex::Regex;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: String,
    }

    let re = Regex::new(r"^ABC[0-9]{3}$").unwrap();
    if !re.is_match(&s) {
        return false;
    }

    let id = s[3..].parse::<usize>().unwrap();
    1 <= id && id <= 349 && id != 316
}

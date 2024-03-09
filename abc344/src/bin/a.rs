use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"\|.*\|").unwrap();
    println!("{}", re.replace(&s, ""));
}

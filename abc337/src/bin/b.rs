use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"^A*B*C*$").unwrap();
    let ans = re.is_match(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}

use proconio::input;
use regex::Regex;

fn main() {
    input! {
        t: usize,
        ns: [(usize, String); t],
    }

    let re = Regex::new(r"^A*B+$").unwrap();

    for (_, s) in &ns {
        println!("{}", if re.is_match(s) { "B" } else { "A" });
    }
}

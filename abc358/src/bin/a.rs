use proconio::input;

fn main() {
    input! {
        (s, t): (String, String),
    }

    println!(
        "{}",
        if s == "AtCoder" && t == "Land" {
            "Yes"
        } else {
            "No"
        }
    );
}

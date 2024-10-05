use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!(
        "{}",
        if &s[s.len() - 3..] == "san" {
            "Yes"
        } else {
            "No"
        }
    );
}

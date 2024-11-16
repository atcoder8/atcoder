use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars,
    }

    n.sort_unstable();

    println!(
        "{}",
        if n == ['1', '2', '2', '3', '3', '3'] {
            "Yes"
        } else {
            "No"
        }
    );
}

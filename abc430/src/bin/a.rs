use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (u8, u8, u8, u8),
    }

    let invalid = c >= a && d < b;
    println!("{}", if invalid { "Yes" } else { "No" });
}

use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!(
        "{}",
        if 2 * a == b || 2 * a + 1 == b {
            "Yes"
        } else {
            "No"
        }
    );
}

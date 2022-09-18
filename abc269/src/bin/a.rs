use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (i64, i64, i64, i64),
    }

    println!("{}", (a + b) * (c - d));
    println!("Takahashi");
}

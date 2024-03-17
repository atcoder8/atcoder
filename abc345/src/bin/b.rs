use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    let ans = if x >= 0 { (x + 9) / 10 } else { -(-x / 10) };
    println!("{}", ans);
}

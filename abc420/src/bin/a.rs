use proconio::input;

fn main() {
    input! {
        (x, y): (u8, u8),
    }

    println!("{}", (x - 1 + y) % 12 + 1);
}

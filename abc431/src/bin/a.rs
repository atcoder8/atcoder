use proconio::input;

fn main() {
    input! {
        (h, b): (u32, u32),
    }

    println!("{}", h.saturating_sub(b));
}

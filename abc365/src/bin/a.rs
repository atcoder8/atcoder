use proconio::input;

fn main() {
    input! {
        y: usize,
    }

    let days = 365 + (y % 400 == 0 || y % 100 != 0 && y % 4 == 0) as usize;
    println!("{}", days);
}

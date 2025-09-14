use proconio::input;

fn main() {
    input! {
        (x, c): (usize, usize),
    }

    println!("{}", 1000 * (x / (1000 + c)));
}

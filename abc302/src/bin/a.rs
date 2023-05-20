use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", (a + b - 1) / b);
}

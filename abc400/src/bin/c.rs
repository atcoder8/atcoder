use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", (n / 2).sqrt() + (n / 4).sqrt());
}

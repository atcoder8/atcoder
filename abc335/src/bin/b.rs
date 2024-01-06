use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for (x, y, z) in iproduct!(0..=n, 0..=n, 0..=n) {
        if x + y + z <= n {
            println!("{x} {y} {z}");
        }
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n < 1000 {
        println!("{}", n);
    } else if n < 10_000 {
        println!("{}", n / 10 * 10);
    } else if n < 100_000 {
        println!("{}", n / 100 * 100);
    } else if n < 1_000_000 {
        println!("{}", n / 1000 * 1000);
    } else if n < 10_000_000 {
        println!("{}", n / 10000 * 10000);
    } else if n < 100_000_000 {
        println!("{}", n / 100_000 * 100_000);
    } else {
        println!("{}", n / 1_000_000 * 1_000_000);
    }
}

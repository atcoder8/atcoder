use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut ten = 1;
    while n >= 1000 {
        n /= 10;
        ten *= 10;
    }
    n *= ten;

    println!("{}", n);
}

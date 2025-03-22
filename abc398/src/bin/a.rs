use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 0 {
        let minus = "-".repeat((n - 2) / 2);
        println!("{0}=={0}", minus);
    } else {
        let minus = "-".repeat((n - 1) / 2);
        println!("{0}={0}", minus);
    }
}

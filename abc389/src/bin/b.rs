use permutohedron::factorial;
use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let ans = (1..).find(|&n| factorial(n) == x).unwrap();
    println!("{}", ans);
}

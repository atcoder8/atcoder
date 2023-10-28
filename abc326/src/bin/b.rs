use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let is_ok = |x: usize| (x / 100) * (x / 10 % 10) == x % 10;
    let ans = (n..).find(|&x| is_ok(x)).unwrap();
    println!("{}", ans);
}

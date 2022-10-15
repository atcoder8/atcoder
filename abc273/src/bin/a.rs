use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn solve(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * solve(n - 1)
    }
}

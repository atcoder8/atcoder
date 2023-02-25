use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (_n, a, b, c, d): (usize, usize, usize, usize, usize),
    }

    if b == 0 && c == 0 && a != 0 && d != 0 {
        return false;
    }

    abs_diff(b, c) <= 1
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

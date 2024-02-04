use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        println!("{}", if solve() { "Yes" } else { "No" });
    }
}

fn solve() -> bool {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    if a > n || n.pow(2) < 2 * a * n - a.pow(2) {
        return false;
    }

    let rem_row = if n > 2 * a {
        a + (n - 2 * a + 1) / 2
    } else {
        n - a
    };
    let rem_col = n - a;

    rem_col * rem_row >= b
}

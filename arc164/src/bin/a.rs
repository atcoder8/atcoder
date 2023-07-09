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
        (n, k): (usize, usize),
    }

    let mut min = 0;
    let mut t = n;
    while t != 0 {
        min += t % 3;
        t /= 3;
    }

    min <= k && k <= n && k % 2 == min % 2
}

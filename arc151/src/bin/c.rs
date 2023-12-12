use proconio::input;

fn main() {
    println!("{}", if solve() { "Takahashi" } else { "Aoki" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        xy: [(usize, usize); m],
    }

    if m == 0 {
        return n % 2 != 0;
    }

    let mut grundy = xy[0].0 - 1 ^ n - xy[m - 1].0;
    for window in xy.windows(2) {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];

        if x1 + 1 != x2 && y1 == y2 {
            grundy ^= 1;
        }
    }

    grundy != 0
}

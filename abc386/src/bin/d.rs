use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        mut xyc: [(Usize1, Usize1, char); m],
    }

    xyc.sort_unstable_by_key(|&(x, _y, c)| (x, c == 'B'));

    let mut border = n;
    for &(_x, y, c) in &xyc {
        if c == 'W' {
            border = border.min(y);
        } else {
            if y >= border {
                return false;
            }
        }
    }

    true
}

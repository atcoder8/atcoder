use proconio::input;

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, d): (usize, usize),
        tt: [usize; n],
    }

    for i in 1..n {
        if tt[i] - tt[i - 1] <= d {
            return Some(tt[i]);
        }
    }

    None
}

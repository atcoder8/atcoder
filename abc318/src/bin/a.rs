use proconio::input;

fn main() {
    input! {
        (n, m, p): (usize, usize, usize),
    }

    let ans = (n.saturating_sub(m - 1) + p - 1) / p;
    println!("{}", ans);
}

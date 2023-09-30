use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        s: String,
        t: String,
    }

    let prefix = s == t[..n];
    let suffix = s == t[(m - n)..];
    let ans = match (prefix, suffix) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };
    println!("{}", ans);
}

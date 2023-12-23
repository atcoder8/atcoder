use proconio::input;

fn main() {
    input! {
        mut a: i64,
        m: i64,
        mut l: i64,
        mut r: i64,
    }

    a -= l;
    a = ((a - m) % m + m) % m;
    r -= l;

    let mut ans = 0;
    if r >= a {
        ans += (r - a) / m + 1;
    }

    println!("{}", ans);
}

use proconio::input;

fn main() {
    input! {
        (n, s, k): (usize, usize, usize),
        pq: [(usize, usize); n],
    }

    let mut ans = 0;
    for (p, q) in pq {
        ans += p * q;
    }
    if ans < s {
        ans += k;
    }

    println!("{}", ans);
}

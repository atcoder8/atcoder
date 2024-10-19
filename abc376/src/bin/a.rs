use proconio::input;

fn main() {
    input! {
        (n, c): (usize, i64),
        tt: [i64; n],
    }

    let mut ans = 0_u32;
    let mut prev = -1000;
    for &t in &tt {
        if t - prev >= c {
            ans += 1;
            prev = t;
        }
    }
    println!("{}", ans);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(usize, usize); n],
    }

    let mut ans = 0_usize;
    let mut prev_t = 0_usize;
    for &(t, v) in &tv {
        ans = ans.saturating_sub(t - prev_t) + v;
        prev_t = t;
    }
    println!("{}", ans);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        m: usize,
        bb: [usize; m],
        x: usize,
    }

    let mut dp = vec![false; x + 1];
    dp[0] = true;

    let mut sticky = vec![false; x];
    for &b in &bb {
        sticky[b] = true;
    }

    for i in 0..x {
        if !dp[i] || sticky[i] {
            continue;
        }

        for &a in &aa {
            if i + a <= x {
                dp[i + a] = true;
            }
        }
    }

    println!("{}", if dp[x] { "Yes" } else { "No" });
}

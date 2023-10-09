use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        dd: [usize; 3],
        aa: [usize; n],
    }

    let mut dp = vec![10_usize.pow(18); 8];
    dp[0] = 0;
    for &a in &aa {
        let mut next_dp = dp.clone();

        for bit in 0..7 {
            let rem = 7 ^ bit;
            let mut add = rem;
            while add != 0 {
                let d = (0..3)
                    .filter(|&i| (add >> i) & 1 == 1)
                    .fold(1, |acc, i| acc.lcm(&dd[i]));
                next_dp[bit | add] = next_dp[bit | add].min(dp[bit] + (d - a % d) % d);

                add = (add - 1) & rem;
            }
        }

        dp = next_dp;
    }

    println!("{}", dp[7]);
}

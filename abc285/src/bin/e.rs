// unfinished

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.insert(0, 0);
    aa[n] = 0;

    let mut dp = vec![(0_usize, n); n];

    for day in 0..n {
        let mut next_dp = dp.clone();

        for i in 0..=day {
            let mut diff = if i > day { i - day } else { day - i };
            diff = diff.min(n - diff);

            if diff >= next_dp[day].1 {
                continue;
            }

            let next_val = dp[day].0 + aa[diff] - aa[dp[day].1];
            if next_val > next_dp[day].0 {
                next_dp[day] = (next_val, diff);
            }
        }

        dp = next_dp;

        println!("dp = {:?}", dp);
    }

    let ans: usize = dp.iter().map(|x| x.0).sum();
    println!("{}", ans);
}

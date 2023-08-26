use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(usize, usize, usize); n],
    }

    let mut takahashi = 0;
    let mut aoki = 0;
    for &(x, y, z) in &xyz {
        if x > y {
            takahashi += z;
        } else {
            aoki += z;
        }
    }

    if takahashi > aoki {
        println!("0");
        std::process::exit(0);
    }

    let req = (aoki - takahashi + 1) / 2;

    // dp[i]: 議席の変化が丁度iとなるのに必要な鞍替え数の最小値
    let sum_z: usize = xyz.iter().map(|x| x.2).sum();
    let mut dp = vec![2e18 as usize; sum_z + 1];
    dp[0] = 0;
    for &(x, y, z) in &xyz {
        if x > y {
            continue;
        }

        let mut next_dp = dp.clone();
        for i in z..=sum_z {
            next_dp[i] = next_dp[i].min(dp[i - z] + (y - x + 1) / 2);
        }
        dp = next_dp;
    }

    let ans = *dp[req..].iter().min().unwrap();
    println!("{}", ans);
}

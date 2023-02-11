use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        cc: [usize; n],
        xx: [Usize1; m],
    }

    let mut req = vec![false; n];
    for &x in &xx {
        req[x] = true;
    }

    let mut dp: Vec<Option<usize>> = vec![None; n + 1];
    dp[0] = Some(0);

    for item_idx in 0..n {
        let mut next_dp: Vec<Option<usize>> = if req[item_idx] {
            vec![None; n + 1]
        } else {
            dp.clone()
        };

        for buy_cnt in 0..=item_idx {
            if let Some(min_acc_cost) = dp[buy_cnt] {
                let next_acc_cost = min_acc_cost + aa[item_idx] + cc[item_idx - buy_cnt];

                if next_dp[buy_cnt + 1].is_none() || next_acc_cost < next_dp[buy_cnt + 1].unwrap() {
                    next_dp[buy_cnt + 1] = Some(next_acc_cost);
                }
            }
        }

        dp = next_dp;

        println!("dp = {:?}", dp);
    }

    let ans = dp.iter().filter_map(|&x| x).min().unwrap();
    println!("{}", ans);
}

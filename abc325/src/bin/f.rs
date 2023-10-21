use proconio::input;

fn main() {
    input! {
        n: usize,
        dd: [usize; n],
        (l1, c1, k1): (usize, usize, usize),
        (l2, c2, k2): (usize, usize, usize),
    }

    // dp[i]: センサー1をi個使ったときに必要なセンサー2の最小個数
    let mut dp = vec![Some(0); k1 + 1];
    for &d in &dd {
        let mut next_dp = vec![None; k1 + 1];
        for from in 0..=k1 {
            let censer2_num = match dp[from] {
                Some(censer2_num) => censer2_num,
                None => continue,
            };

            for to in from..=k1 {
                let add_censer1_num = to - from;
                let add_censer2_num = (d.saturating_sub(l1 * add_censer1_num) + l2 - 1) / l2;
                let candidate_censer2_num = censer2_num + add_censer2_num;

                if candidate_censer2_num > k2 {
                    continue;
                }

                let next_censer2_num = &mut next_dp[to];
                if next_censer2_num.is_none() || candidate_censer2_num < next_censer2_num.unwrap() {
                    *next_censer2_num = Some(candidate_censer2_num);
                }
            }
        }

        dp = next_dp;
    }

    let ans = dp
        .iter()
        .enumerate()
        .filter_map(|(censer1_num, &censer2_num)| match censer2_num {
            Some(censer2_num) => Some(censer1_num * c1 + censer2_num * c2),
            None => None,
        })
        .min();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

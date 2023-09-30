use proconio::input;

fn main() {
    input! {
        (n, k, p): (usize, usize, usize),
        caa: [(usize, [usize; k]); n],
    }

    let to_params = |mut x: usize| {
        let mut params = vec![];
        for _ in 0..k {
            params.push(x % (p + 1));
            x /= p + 1;
        }

        params
    };

    let to_idx = |params: &[usize]| {
        let mut idx = 0;
        for i in 0..k {
            idx += params[i] * (p + 1).pow(i as u32);
        }

        idx
    };

    let mut dp = vec![None; (p + 1).pow(k as u32)];
    dp[0] = Some(0);
    for (c, aa) in caa {
        let mut next_dp = dp.clone();

        for cur in 0..(p + 1).pow(k as u32) {
            let cur_cost = match dp[cur] {
                Some(cur_cost) => cur_cost,
                None => continue,
            };

            let mut params = to_params(cur);
            for i in 0..k {
                params[i] = (params[i] + aa[i]).min(p);
            }

            let next = to_idx(&params);
            if next_dp[next].is_none() || cur_cost + c < next_dp[next].unwrap() {
                next_dp[next] = Some(cur_cost + c);
            }
        }

        dp = next_dp;
    }

    match dp[(p + 1).pow(k as u32) - 1] {
        Some(ans) => println!("{ans}"),
        None => println!("-1"),
    }
}

use proconio::input;

fn main() {
    input! {
        (n, h): (usize, usize),
        mut xx: [usize; n],
        mut pf: [(usize, usize); n - 1],
    }

    xx.insert(0, 0);
    pf.push((0, 0));

    let update = |next_cost: &mut Option<usize>, candidate_cost: usize| {
        if next_cost.is_none() || candidate_cost < next_cost.unwrap() {
            *next_cost = Some(candidate_cost);
        }
    };

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; h + 1]; h + 1];
    dp[h][0] = Some(0);
    for i in 0..n {
        let mut next_dp: Vec<Vec<Option<usize>>> = vec![vec![None; h + 1]; h + 1];

        let dist = xx[i + 1] - xx[i];
        let (p, f) = pf[i];

        for rem in dist..=h {
            for req in 0..(h + 1).saturating_sub(dist) {
                let cost = match dp[rem][req] {
                    Some(candidate_cost) => candidate_cost,
                    None => continue,
                };

                update(&mut next_dp[rem - dist][req + dist], cost);
                update(&mut next_dp[(rem - dist + f).min(h)][req + dist], cost + p);
                update(
                    &mut next_dp[rem - dist][(req + dist).saturating_sub(f)],
                    cost + p,
                );
            }
        }

        dp = next_dp;
    }

    let mut min_cost = None;
    for rem in 0..=h {
        for req in 0..=rem {
            if let Some(cost) = dp[rem][req] {
                if min_cost.is_none() || cost < min_cost.unwrap() {
                    min_cost = Some(cost);
                }
            }
        }
    }

    if let Some(ans) = min_cost {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

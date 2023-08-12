use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        css: [(f64, [usize]); n],
    }

    let mut dp = vec![0.0; m + 1];
    for i in (0..m).rev() {
        let mut min_exp = None;

        for (c, ss) in &css {
            let p = ss.len() as f64;
            let mut zero_cnt = 0;
            let mut exp = 0.0;

            for &s in ss {
                if s == 0 {
                    zero_cnt += 1;
                    exp += *c / p;
                } else {
                    exp += (dp[(i + s).min(m)] + *c) / p;
                }
            }

            exp *= p / (p - zero_cnt as f64);

            if min_exp.is_none() || exp < min_exp.unwrap() {
                min_exp = Some(exp);
            }
        }

        dp[i] = min_exp.unwrap();
    }

    println!("{}", dp[0]);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        dd: [usize; n * (n - 1) / 2],
    }

    let mut weights = vec![vec![0; n]; n];
    let mut idx = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            weights[i][j] = dd[idx];
            weights[j][i] = dd[idx];
            idx += 1;
        }
    }

    let mut dp: Vec<Option<usize>> = vec![None; 1 << n];
    dp[0] = Some(0);
    for bit in 0..(1 << n) {
        let score = match dp[bit] {
            Some(score) => score,
            None => continue,
        };

        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                continue;
            }

            for j in (i + 1)..n {
                if (bit >> j) & 1 == 1 {
                    continue;
                }

                let candidate_score = score + weights[i][j];
                let next_score = &mut dp[bit | (1 << i) | (1 << j)];
                if next_score.is_none() || candidate_score > next_score.unwrap() {
                    *next_score = Some(candidate_score);
                }
            }
        }
    }

    let ans = dp.iter().filter_map(|&x| x).max().unwrap();
    println!("{}", ans);
}

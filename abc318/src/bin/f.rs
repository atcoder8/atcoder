use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xx: [i64; n],
        ll: [i64; n],
    }

    let mut ans = 0;
    let mut k = xx[n - 1] - ll[n - 1];
    while k <= xx[0] + ll[n - 1] {
        let dists = xx
            .iter()
            .map(|&x| (x - k).abs())
            .sorted_by_key(|&dist| dist)
            .collect_vec();

        match calc_min_margin(&dists, &ll) {
            Ok(min_margin) => {
                ans += min_margin + 1;
                k += min_margin + 1;
            }
            Err(lack) => {
                k += lack;
            }
        }
    }

    println!("{}", ans);
}

fn calc_min_margin(dists: &[i64], lengths: &[i64]) -> Result<i64, i64> {
    let mut min_margin = *lengths.last().unwrap();

    for (&dist, &len) in dists.iter().zip(lengths) {
        if dist <= len {
            min_margin = min_margin.min(len - dist);
        } else {
            return Err(dist - len);
        }
    }

    Ok(min_margin)
}

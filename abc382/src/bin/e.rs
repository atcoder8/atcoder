use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        mut pp: [f64; n],
    }

    pp.iter_mut().for_each(|p| *p = 0.01 * *p);

    // レアカードの枚数ごとの確率
    let mut prob_by_rare = vec![0.0; n + 1];
    prob_by_rare[0] = 1.0;
    let mut next_prob_by_rare = vec![0.0; n + 1];
    for &p in &pp {
        for (i, &prob) in enumerate(&prob_by_rare[..n]) {
            next_prob_by_rare[i + 1] += prob * p;
            next_prob_by_rare[i] += prob * (1.0 - p);
        }

        std::mem::swap(&mut prob_by_rare, &mut next_prob_by_rare);
        next_prob_by_rare.fill(0.0);
    }

    let prob_empty = prob_by_rare[0];
    let mut exp_by_rare = vec![0.0; x + 1];
    for cur in (0..x).rev() {
        exp_by_rare[cur] = (1.0
            + (1..=n)
                .map(|add| prob_by_rare[add] * exp_by_rare[(cur + add).min(x)])
                .sum::<f64>())
            / (1.0 - prob_empty);
    }

    println!("{}", exp_by_rare[0]);
}

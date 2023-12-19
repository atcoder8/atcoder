use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let frequencies = aa
        .into_iter()
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();
    let max_freq = frequencies.iter().map(|x| x.0).max().unwrap();
    let modes = frequencies
        .iter()
        .filter(|x| x.0 == max_freq)
        .map(|x| x.1)
        .collect_vec();

    let mut ans = (n - max_freq) * k + modes[0] - modes.last().unwrap();
    if modes.len() >= 2 {
        let cand = (n - max_freq - 1) * k
            + modes
                .windows(2)
                .map(|window| window[1] - window[0])
                .max()
                .unwrap();
        ans = ans.max(cand);
    }

    println!("{}", ans);
}

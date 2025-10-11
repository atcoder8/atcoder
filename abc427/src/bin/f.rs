use std::collections::BTreeMap;

use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    if n == 1 {
        return 1 + (aa[0] % m == 0) as usize;
    }

    let calc_combs = |aa: &[usize]| {
        let mut inclusive = BTreeMap::<usize, usize>::new();
        let mut exclusive = BTreeMap::<usize, usize>::new();
        exclusive.insert(0, 1);
        for &a in aa {
            let mut next_inclusive = BTreeMap::<usize, usize>::new();
            let mut next_exclusive = exclusive.clone();
            for (&rem, &num_combs) in &inclusive {
                *next_exclusive.entry(rem).or_default() += num_combs;
            }
            for (&rem, &num_combs) in &exclusive {
                let next_rem = (rem + a) % m;
                *next_inclusive.entry(next_rem).or_default() += num_combs;
            }

            inclusive = next_inclusive;
            exclusive = next_exclusive;
        }

        (inclusive, exclusive)
    };

    let (half1, half2) = aa.split_at(n / 2);
    let half1 = half1.to_owned();
    let mut half2 = half2.to_owned();
    half2.reverse();

    let (inclusive1, exclusive1) = calc_combs(&half1);
    let (inclusive2, exclusive2) = calc_combs(&half2);

    let mut sum = 0_usize;
    for (&rem1, &cnt1) in &inclusive1 {
        let rem2 = (m - rem1) % m;
        if let Some(cnt2) = exclusive2.get(&rem2) {
            sum += cnt1 * cnt2;
        }
    }
    for (&rem1, &cnt1) in &exclusive1 {
        let rem2 = (m - rem1) % m;
        if let Some(cnt2) = inclusive2.get(&rem2) {
            sum += cnt1 * cnt2;
        }
        if let Some(cnt2) = exclusive2.get(&rem2) {
            sum += cnt1 * cnt2;
        }
    }

    sum
}

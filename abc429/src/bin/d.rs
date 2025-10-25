use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m, c): (usize, usize, usize),
        aa: [usize; n],
    }

    let counts = aa
        .iter()
        .cloned()
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();

    let mut sum = 0_usize;
    let mut right = 0_usize;
    let mut num_encounters = counts[0].0;
    for left in 0..counts.len() {
        num_encounters -= counts[left].0;

        while num_encounters < c {
            right += 1;
            num_encounters += counts[right % counts.len()].0;
        }

        let position1 = counts[left].1;
        let position2 = counts[(left + 1) % counts.len()].1;
        let interval = if position1 < position2 {
            position2 - position1
        } else {
            position2 + m - position1
        };
        sum += num_encounters * interval;
    }

    println!("{sum}");
}

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        xx: [Usize1; m],
    }

    let mut base_dist = 0;
    let mut add_lengths = vec![0; n + 1];
    for (mut x1, mut x2) in xx.iter().cloned().tuple_windows() {
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }

        let dist1 = (x2 - x1) as i64;
        let dist2 = (n - (x2 - x1)) as i64;

        if dist1 <= dist2 {
            base_dist += dist1;

            let diff_dist = dist2 - dist1;
            add_lengths[x1] += diff_dist;
            add_lengths[x2] -= diff_dist;
        } else {
            base_dist += dist2;

            let diff_dist = dist1 - dist2;
            add_lengths[0] += diff_dist;
            add_lengths[n] -= diff_dist;
            add_lengths[x1] -= diff_dist;
            add_lengths[x2] += diff_dist;
        }
    }
    for i in 0..n {
        add_lengths[i + 1] += add_lengths[i];
    }

    let ans = base_dist + add_lengths[..n].iter().min().unwrap();
    println!("{}", ans);
}

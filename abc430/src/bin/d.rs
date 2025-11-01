use std::collections::BTreeMap;

use proconio::{fastout, input};

const MAX: usize = 10_usize.pow(9);

#[fastout]
fn main() {
    input! {
        n: usize,
        xx: [usize; n],
    }

    let mut sum_min_dist = MAX + 1;
    let mut map = BTreeMap::from([(0, MAX + 1)]);
    for &x in &xx {
        let (&left, dist) = map.range_mut(..x).next_back().unwrap();
        let cand_dist = x - left;
        if cand_dist < *dist {
            sum_min_dist -= *dist - cand_dist;
            *dist = cand_dist;
        }

        let mut curr_dist = cand_dist;

        if let Some((&right, dist)) = map.range_mut(x + 1..).next() {
            let cand_dist = right - x;
            if cand_dist < *dist {
                sum_min_dist -= *dist - cand_dist;
                *dist = cand_dist;
            }

            curr_dist = curr_dist.min(cand_dist);
        }

        map.insert(x, curr_dist);

        sum_min_dist += curr_dist;

        println!("{sum_min_dist}");
    }
}

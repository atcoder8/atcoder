use im_rc::HashSet;
use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t)
        .map(|_| if solve() { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}

fn solve() -> bool {
    input! {
        (n, k): (usize, usize),
        aa: [Usize1; n],
        bb: [Usize1; n],
    }

    if aa == bb {
        return true;
    }

    if k == 1 {
        let mut iter_a = aa.iter();
        return bb
            .iter()
            .dedup()
            .all(|&b| iter_a.find(|&&a| a == b).is_some());
    }

    let set_a: HashSet<usize> = aa.iter().cloned().collect();
    let set_b: HashSet<usize> = bb.iter().cloned().collect();

    if set_b.len() == n || !set_b.is_subset(&set_a) {
        return false;
    }

    let mut positions = vec![None::<usize>; n];
    for (i, &b) in enumerate(&bb) {
        if let Some(prev_pos) = positions[b].take() {
            if i - prev_pos <= k {
                return true;
            }
        }

        positions[b] = Some(i);
    }

    false
}

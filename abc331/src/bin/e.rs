use std::cmp::Reverse;

use im_rc::HashSet;
use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, l): (usize, usize, usize),
        aa: [usize; n],
        bb: [usize; m],
        cd: [(Usize1, Usize1); l],
    }

    let ia = enumerate(aa.clone())
        .sorted_unstable_by_key(|x| Reverse(x.1))
        .collect_vec();
    let jb = enumerate(bb.clone())
        .sorted_unstable_by_key(|x| Reverse(x.1))
        .collect_vec();

    let unsupported: HashSet<(usize, usize)> = cd.iter().cloned().collect();

    let is_ok = |key: usize| {
        for &(i, a) in &ia {
            for &(j, b) in &jb {
                if a + b < key {
                    break;
                }

                if !unsupported.contains(&(i, j)) {
                    return true;
                }
            }
        }

        false
    };

    let mut ok = 0;
    let mut ng = 10_usize.pow(10);
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

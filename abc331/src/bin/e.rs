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

    let ia = enumerate(aa)
        .sorted_unstable_by_key(|x| Reverse(x.1))
        .collect_vec();
    let jb = enumerate(bb)
        .sorted_unstable_by_key(|x| Reverse(x.1))
        .collect_vec();

    let unsupported: HashSet<(usize, usize)> = cd.iter().cloned().collect();

    let mut ans = 0;
    for &(i, a) in &ia {
        for &(j, b) in &jb {
            if !unsupported.contains(&(i, j)) {
                ans = ans.max(a + b);
                break;
            }
        }
    }

    println!("{}", ans);
}

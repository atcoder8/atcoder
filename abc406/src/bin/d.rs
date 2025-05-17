use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        xy: [(Usize1, Usize1); n],
        q: usize,
        queries: [(usize, Usize1); q],
    }

    let mut horizontal = vec![BTreeSet::new(); h];
    let mut vertical = vec![BTreeSet::new(); w];
    for &(x, y) in &xy {
        horizontal[x].insert(y);
        vertical[y].insert(x);
    }

    for (qt, sub_coord) in queries {
        if qt == 1 {
            let line = std::mem::take(&mut horizontal[sub_coord]);
            println!("{}", line.len());
            for y in line {
                vertical[y].remove(&sub_coord);
            }
        } else {
            let line = std::mem::take(&mut vertical[sub_coord]);
            println!("{}", line.len());
            for x in line {
                horizontal[x].remove(&sub_coord);
            }
        }
    }
}

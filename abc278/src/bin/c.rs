use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (_n, q): (usize, usize),
        tab: [(usize, Usize1, Usize1); q],
    }

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (t, a, b) in tab {
        if t == 1 {
            map.entry(a).or_insert(HashSet::new()).insert(b);
        } else if t == 2 {
            map.entry(a).or_insert(HashSet::new()).remove(&b);
        } else {
            let ans = map.entry(a).or_insert(HashSet::new()).contains(&b)
                && map.entry(b).or_insert(HashSet::new()).contains(&a);
            println!("{}", if ans { "Yes" } else { "No" });
        }
    }
}

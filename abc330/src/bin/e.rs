use std::collections::BTreeSet;

use hashbag::HashBag;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [usize; n],
        ix: [(Usize1, usize); q],
    }

    let mut counter = HashBag::new();
    for &a in &aa {
        counter.insert(a);
    }

    let mut excluded = BTreeSet::new();
    for i in 0..=n {
        if counter.contains(&i) == 0 {
            excluded.insert(i);
        }
    }

    for &(i, x) in &ix {
        let a = &mut aa[i];

        counter.remove(a);
        if counter.contains(a) == 0 {
            excluded.insert(*a);
        }

        counter.insert(x);
        excluded.remove(&x);

        *a = x;

        println!("{}", excluded.first().unwrap());
    }
}

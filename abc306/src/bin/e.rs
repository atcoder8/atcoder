use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k, q): (usize, usize, usize),
        xy: [(Usize1, usize); q],
    }

    let mut fa = 0;
    let mut aa = vec![0_usize; n];
    let mut bb: BTreeMap<usize, usize> = BTreeMap::new();
    let mut rem = BTreeMap::new();
    bb.insert(0, k);
    rem.insert(0, n - k);

    for &(x, y) in &xy {
        let elem = &mut aa[x];

        if let Some(cnt) = rem.get_mut(elem) {
            *cnt -= 1;
            if *cnt == 0 {
                rem.remove(elem);
            }
        } else {
            if let Some(cnt) = bb.get_mut(elem) {
                fa -= *elem;
                *cnt -= 1;
                if *cnt == 0 {
                    bb.remove(elem);
                }
            }

            if let Some((&max_key, max_value)) = rem.iter_mut().next_back() {
                fa += max_key;
                *bb.entry(max_key).or_insert(0) += 1;
                *max_value -= 1;
                if *max_value == 0 {
                    rem.remove(&max_key);
                }
            }
        }

        *elem = y;

        *bb.entry(y).or_insert(0) += 1;
        fa += y;

        let (&min_key, min_value) = bb.iter_mut().next().unwrap();
        fa -= min_key;
        *min_value -= 1;
        if *min_value == 0 {
            bb.remove(&min_key);
        }
        *rem.entry(min_key).or_insert(0) += 1;

        println!("{}", fa);
    }
}

use std::collections::BTreeMap;

use im_rc::HashMap;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [usize; n],
        ix: [(Usize1, usize); q],
    }

    let mut counter: HashMap<usize, usize> = HashMap::new();
    let mut map = BTreeMap::new();
    for &a in &aa {
        *counter.entry(a).or_default() += 1;
        if counter[&a] == 1 {
            insert(&mut map, a);
        }
    }

    for &(i, x) in &ix {
        let cnt = counter.get_mut(&aa[i]).unwrap();
        *cnt -= 1;
        if *cnt == 0 {
            counter.remove(&aa[i]);
            remove(&mut map, aa[i]);
        }

        *counter.entry(x).or_default() += 1;
        if counter[&x] == 1 {
            insert(&mut map, x);
        }

        aa[i] = x;

        let (left, right) = map.iter().next().unwrap();
        let ans = if *left == 0 { *right + 1 } else { 0 };
        println!("{}", ans);
    }
}

fn insert(map: &mut BTreeMap<usize, usize>, x: usize) {
    let mut r = x;

    if let Some(right) = map.remove(&(x + 1)) {
        r = right;
    }

    if let Some((_, right)) = map.range_mut(..=x).next_back() {
        if *right + 1 >= x {
            *right = (*right).max(r);
        } else {
            map.insert(x, r);
        }
    } else {
        map.insert(x, r);
    }
}

fn remove(map: &mut BTreeMap<usize, usize>, x: usize) {
    if let Some(right) = map.remove(&x) {
        map.insert(x + 1, right);
        return;
    }

    let (left, right) = map.range_mut(..=x).next_back().unwrap();
    if *right == x {
        if left == right {
            map.remove(&x);
        } else {
            *right -= 1;
        }
    } else {
        let t = *right;
        *right = x - 1;
        map.insert(x + 1, t);
    }
}

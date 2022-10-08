use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [i64; n],
    }

    // vals[i]: (i+1)回目の操作後に存在する非負整数の集合
    let mut vals: Vec<HashSet<usize>> = vec![HashSet::new(); m + 1];

    for (i, &a) in aa.iter().enumerate() {
        let l = if a >= 0 {
            1
        } else {
            (-a as usize + i) / (i + 1)
        };

        for j in l..=(m + 1) {
            let added = aa[i] + (i as i64 + 1) * j as i64;

            if 0 <= added && added <= n as i64 {
                vals[j - 1].insert(added as usize);
            } else {
                break;
            }
        }
    }

    for i in 0..m {
        for j in 0..=n {
            if !vals[i].contains(&j) {
                println!("{}", j);
                break;
            }
        }
    }
}

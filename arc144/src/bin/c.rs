use std::collections::BTreeSet;

use itertools::join;

fn main() {
    let (n, k): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    if 2 * k > n {
        println!("-1");
        std::process::exit(0);
    }

    let mut bts = BTreeSet::new();
    for i in 0..n {
        bts.insert(i + 1);
    }

    let mut aa = vec![0; n];
    for i in 0..k {
        aa[i] = k + 1 + i;
        bts.remove(&(k + 1 + i));
    }

    let after = (n - 2 * k).min(k);
    for i in 0..after {
        aa[n - k - after + i] = n - after + 1 + i;
        bts.remove(&(n - after + 1 + i));
    }

    for i in 0..n {
        if aa[i] != 0 {
            continue;
        }

        let first = *bts.iter().next().unwrap();
        if i + 1 - first >= k {
            aa[i] = first;
            bts.remove(&first);
        } else {
            aa[i] = *bts.range((i + 1 + k)..).next().unwrap();
            bts.remove(&aa[i]);
        }
    }

    println!("{}", join(aa.iter(), " "));
}

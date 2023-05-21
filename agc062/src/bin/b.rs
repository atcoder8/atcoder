// unfinished

// use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        mut cc: [usize; k],
        pp: [Usize1; n],
    }

    // let mut kk = vec![];
    // for i in 0..(n - 1) {
    //     if pp[i] > pp[i + 1] {
    //         kk.push(n - (i + 1));
    //     }
    // }

    // if kk.len() > k {
    //     println!("-1");
    //     std::process::exit(0);
    // }

    // cc.sort_unstable();
    // kk.sort_unstable();
    // kk.resize(k, 0);
    // println!("kk = {:?}", kk);

    // let ans: usize = cc.iter().zip(&kk).map(|(&c, &k)| c * k).sum();
    // println!("{}", ans);

    let mut sequences = vec![];
    let mut used = vec![false; n];

    for i in 0..n {
        if used[i] {
            continue;
        }

        let mut seq = vec![pp[i]];
        used[i] = true;
        for j in (i + 1)..n {
            if !used[j] && pp[j] > *seq.last().unwrap() {
                seq.push(pp[j]);
                used[j] = true;
            }
        }

        sequences.push(seq);
    }

    println!("sequences = {:?}", sequences);

    let mut positions = sequences
        .iter()
        .map(|seq| *seq.last().unwrap() + 1)
        .collect_vec();
    positions.sort_unstable();

    if positions.len() > k {
        println!("-1");
        std::process::exit(0);
    }

    positions.resize(k, 0);
    println!("positions = {:?}", positions);
    cc.sort_unstable();

    let ans: usize = cc.iter().zip(&positions).map(|(&c, &pos)| c * pos).sum();
    println!("{}", ans);
}

// fn tle(input: (Vec<usize>, Vec<usize>)) -> Option<usize> {
//     let (c, p) = input;

//     let mut queue = VecDeque::from(vec![p]);

//     todo!()
// }

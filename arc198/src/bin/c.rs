// unfinished

// use std::collections::BTreeMap;

use proconio::input;

fn main() {
    // let mut stack: Vec<([u8; 3], u8)> = vec![([10, 20, 30], 0)];
    // let mut pool = BTreeMap::<[u8; 3], u8>::new();
    // while let Some((seq, cost)) = stack.pop() {
    //     if pool.contains_key(&seq) {
    //         continue;
    //     }

    //     pool.insert(seq, cost);

    //     for (i, j) in [(0, 1), (0, 2), (1, 2)] {
    //         let mut next_seq = seq;
    //         next_seq.swap(i, j);
    //         next_seq[i] -= 1;
    //         next_seq[j] += 1;

    //         if next_seq
    //             .iter()
    //             .all(|elem| (elem % 10).min(10 - elem % 10) <= 2)
    //         {
    //             stack.push((next_seq, cost + 1));
    //         }
    //     }
    // }
    // for (seq, cost) in &pool {
    //     if seq.iter().all(|elem| (elem % 10).min(10 - elem % 10) <= 1) {
    //         println!("[{}]: {cost}", seq.iter().join(", "));
    //     }
    // }
    // print!("{}", pool.len());

    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    }

    let sum_a = aa.iter().sum::<usize>();
    let sum_b = bb.iter().sum::<usize>();
    if sum_a != sum_b {
        return false;
    }

    if n == 2 {
        return aa == bb || vec![aa[1] - 1, aa[0] + 1] == bb;
    }

    for pos in 0..n - 1 {
        if aa[pos] == bb[pos] {
            continue;
        }

        if bb[pos] < aa[pos] {
            
        }
    }

    todo!()
}

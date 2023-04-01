use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut inv_aa = vec![vec![]; n];
    for (i, &a) in aa.iter().enumerate() {
        inv_aa[a].push(i);
    }

    let mut doubling = vec![inv_aa];
    for i in (1..).take_while(|i| (2 * n) >> i != 0) {
        let mut next = vec![vec![]; n];
        for j in 0..n {
            for &half in &doubling[i - 1][j] {
                for &dest in &doubling[i - 1][half] {
                    next[j].push(dest);
                }
            }
        }
        doubling.push(next);
    }

    let ans = (0..n).filter(|&i| !doubling.last().unwrap()[i].is_empty()).count();
    println!("{}", ans);
}

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        aaa: [[usize; n]; n],
    }

    let mut comb1 = vec![vec![HashMap::new(); n]; n];
    comb1[0][0].insert(aaa[0][0], 1_usize);

    for i in 0..(n - 1) {
        for j in 0..(n - 1 - i) {
            for (bit, cnt) in comb1[i][j].clone() {
                comb1[i + 1][j]
                    .entry(bit ^ aaa[i + 1][j])
                    .and_modify(|v| *v += cnt)
                    .or_insert(cnt);

                comb1[i][j + 1]
                    .entry(bit ^ aaa[i][j + 1])
                    .and_modify(|v| *v += cnt)
                    .or_insert(cnt);
            }
        }
    }

    let mut comb2 = vec![vec![HashMap::new(); n]; n];
    comb2[n - 1][n - 1].insert(aaa[n - 1][n - 1], 1_usize);

    for i in (1..n).rev() {
        for j in ((n - i)..n).rev() {
            for (bit, cnt) in comb2[i][j].clone() {
                comb2[i - 1][j]
                    .entry(bit ^ aaa[i - 1][j])
                    .and_modify(|v| *v += cnt)
                    .or_insert(cnt);

                comb2[i][j - 1]
                    .entry(bit ^ aaa[i][j - 1])
                    .and_modify(|v| *v += cnt)
                    .or_insert(cnt);
            }
        }
    }

    let mut ans = 0;

    for i in 0..n {
        let j = n - 1 - i;

        for (bit, cnt1) in &comb1[i][j] {
            ans += cnt1 * comb2[i][j].get(&(bit ^ aaa[i][j])).unwrap_or(&0);
        }
    }

    println!("{}", ans);
}

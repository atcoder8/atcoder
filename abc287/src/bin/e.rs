use itertools::{join, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut si = ss
        .iter()
        .enumerate()
        .map(|(i, s)| (s.clone(), i))
        .collect_vec();
    si.sort_unstable();

    let mut ans = vec![0; n];

    for i in 0..n {
        let n1 = if i == 0 {
            0
        } else {
            si[i - 1]
                .0
                .iter()
                .zip(&si[i].0)
                .take_while(|(prev, cur)| prev == cur)
                .count()
        };

        let n2 = if i < n - 1 {
            si[i]
                .0
                .iter()
                .zip(&si[i + 1].0)
                .take_while(|(cur, next)| cur == next)
                .count()
        } else {
            0
        };

        ans[si[i].1] = n1.max(n2);
    }

    println!("{}", join(&ans, "\n"));
}

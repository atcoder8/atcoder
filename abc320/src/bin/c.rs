use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        ss: [Chars; 3],
    }

    let mut ans: Option<usize> = None;
    for i in 0..m {
        let c = ss[0][i];

        for j in 0..m {
            if ss[1][j] != c {
                continue;
            }

            for k in 0..m {
                if ss[2][k] != c {
                    continue;
                }

                let mut tt = [i, j, k];
                tt.sort_unstable();

                let candidate = if tt.iter().all_equal() {
                    tt[0] + 2 * m
                } else if tt.iter().all_unique() {
                    *tt.iter().max().unwrap()
                } else {
                    tt[0] + m
                };

                if ans.is_none() || candidate < ans.unwrap() {
                    ans = Some(candidate);
                }
            }
        }
    }

    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

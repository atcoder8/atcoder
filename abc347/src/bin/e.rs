use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        xx: [Usize1; q],
    }

    let mut ans = vec![0; n];
    let mut insert_times: Vec<Option<usize>> = vec![None; n];
    let mut prefix_sum = vec![0; q + 1];
    let mut cnt = 0_usize;
    for (i, &x) in enumerate(&xx) {
        match insert_times[x] {
            Some(insert_time) => {
                ans[x] += prefix_sum[i] - prefix_sum[insert_time];
                insert_times[x] = None;
                cnt -= 1;
            }
            None => {
                insert_times[x] = Some(i);
                cnt += 1;
            }
        }

        prefix_sum[i + 1] = prefix_sum[i] + cnt;
    }

    for x in 0..n {
        if let Some(insert_time) = insert_times[x] {
            ans[x] += prefix_sum[q] - prefix_sum[insert_time];
        }
    }

    println!("{}", ans.iter().join(" "));
}

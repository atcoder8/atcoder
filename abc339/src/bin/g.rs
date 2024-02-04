use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    let log_n = (0..).find(|i| (n - 1) >> i == 0).unwrap();
    let mut merge_sort_tree = vec![aa; log_n + 1];
    for level in 1..=log_n {
        let seq = &mut merge_sort_tree[level];
        seq.chunks_mut(1 << level)
            .for_each(|sub_seq| sub_seq.sort_unstable());
    }

    let mut prefix_sum_per_level = vec![vec![0; n + 1]; log_n + 1];
    for level in 0..=log_n {
        for i in 0..n {
            prefix_sum_per_level[level][i + 1] =
                prefix_sum_per_level[level][i] + merge_sort_tree[level][i];
        }
    }

    let mut ans = vec![0];
    for &(alpha, beta, gamma) in &queries {
        let b = *ans.last().unwrap();
        let mut left = (alpha ^ b) - 1;
        let mut right = beta ^ b;
        let x = gamma ^ b;

        let mut sum = 0;

        let mut level = 0;
        while left != right {
            let size = 1 << level;
            let sorted = &merge_sort_tree[level];
            let prefix_sum = &prefix_sum_per_level[level];

            if left >> level & 1 == 1 {
                let upper_bound = sorted[left..left + size].upper_bound(&x);
                sum += prefix_sum[left + upper_bound] - prefix_sum[left];
                left += size;
            }

            if right >> level & 1 == 1 {
                let upper_bound = sorted[right - size..right].upper_bound(&x);
                sum += prefix_sum[right - size + upper_bound] - prefix_sum[right - size];
                right -= size;
            }

            level += 1;
        }

        ans.push(sum);
    }

    println!("{}", ans[1..].iter().join("\n"));
}

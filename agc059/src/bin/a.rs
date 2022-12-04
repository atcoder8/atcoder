// Not solved yet.

use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, q): (usize, usize),
        s: Chars,
        lr: [(Usize1, Usize1); q],
    }

    let mut depths = vec![0_usize; n];

    let mut left = 0;
    let mut right = n - 1;
    let mut cnt = 0;

    while left < right {
        depths[left] = cnt;
        while left < right && s[left] == s[left + 1] {
            left += 1;
            depths[left] = cnt;
        }

        if s[left] != s[right] {
            cnt += 1;
        }

        depths[right] = cnt;
        while left < right && s[right] == s[right - 1] {
            right -= 1;
            depths[right] = cnt;
        }

        left += 1;
        right -= 1;

        cnt += 1;

        if left == right {
            depths[left] = cnt;

            break;
        }
    }

    let max_depth_idx = depths.iter().position_max().unwrap();
    let max_depth = depths[max_depth_idx];

    for &(l, r) in &lr {
        if l > max_depth_idx {
            println!("{}", depths[l] - depths[r]);
        } else if r < max_depth_idx {
            println!("{}", depths[r] - depths[l]);
        } else {
            println!("{}", (max_depth - depths[l]).max(max_depth - depths[r]));
        }
    }
}

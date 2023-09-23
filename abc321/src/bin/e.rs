// unfinished

use proconio::input;

fn main() {
    input! {
        nxk: [(usize, usize, usize)]
    }

    for &(n, x, k) in &nxk {
        println!("{}", solve(n, x, k));
    }
}

fn solve(n: usize, x: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }

    let calc_rank = |node: usize| (1_usize..).find(|&i| node >> i == 0).unwrap() - 1;

    let n_rank = calc_rank(n);
    let x_rank = calc_rank(x);

    let mut ans = 0;

    // K代下の子孫
    if k <= 60 && n >= x << k {
        ans += (((x + 1) << k) - 1).min(n) - (x << k) + 1;
    }

    // i代上の祖先で折返し(1<=i<=K-2)
    if k >= 2 {
        if k <= x_rank + 2 {
            ans += 2_usize.pow((k - 1) as u32) - 1;
        }
    }
    let min = if x_rank + k < n_rank {
        1
    } else {
        (x_rank + k - n_rank + 1) / 2 + 1
    };
    let max = (k - 2).min(x_rank);
    if min <= max {
        ans += 2_usize.pow((k - min) as u32) - 1;
        if max < k {
            ans -= 2_usize.pow((k - max) as u32) - 1;
        }
        if k >= 2 * (min - 1) {
            ans += ((x + 1) << (k - 2 * (min - 1))).min(n) - (x << (k - 2 * (min - 1))).min(n) + 1;
        }
    }

    // K-1代上の祖先で折返し
    if x_rank >= k - 1 {
        ans += 1;
    }

    // K代上の祖先で折返し
    if x_rank >= k {
        ans += 1;
    }

    ans
}

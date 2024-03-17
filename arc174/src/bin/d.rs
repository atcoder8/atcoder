// unfinished

use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        t: usize,
        nn: [usize; t],
    }

    let is_ok = |x: usize| {
        let y = x.sqrt();

        let str_x = x.to_string();
        let str_y = y.to_string();

        str_x[..str_y.len()] == str_y[..]
    };

    let solve = |n: usize| {
        let mut ans = 0;
        for exp in 0..=18 {
            let left = 10_usize.pow(exp);

            if left > n {
                break;
            }

            let right = 2 * left;

            let mut ok = left;
            let mut ng = right;
            while ok.abs_diff(ng) > 1 {
                let mid = (ok + ng) / 2;

                if is_ok(mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            ans += ok - left + 1;
        }

        ans
    };

    println!("{}", nn.iter().map(|&n| solve(n)).join("\n"));
}

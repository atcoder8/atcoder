use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [Usize1; n],
    }

    let (left, right) = (|| {
        let mut less = 0;
        let mut large = 0;

        for left in 0..n {
            let head = aa[left];

            let mut bb = aa.clone();
            bb[left..].sort_unstable();

            for &b in &bb[left..] {
                if b >= head {
                    break;
                }

                less += 1;
                if less == k {
                    let right = aa.iter().find_position(|&&a| a == b).unwrap().0;
                    return (left, right);
                }
            }

            for &b in bb[left..].iter().rev() {
                if b <= head {
                    break;
                }

                large += 1;
                if large == n * (n + 1) / 2 - k + 1 {
                    let right = aa.iter().find_position(|&&a| a == b).unwrap().0;
                    return (left, right);
                }
            }
        }

        (0, 0)
    })();

    let mut ans = aa.clone();
    ans[left..=right].reverse();

    println!("{}", join(ans.iter().map(|&x| x + 1), " "));
}

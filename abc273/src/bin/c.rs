use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut bb = aa.clone();
    bb.dedup();
    let m = bb.len();

    let mut ans = vec![0; n];

    for k in 0..n {
        if k >= m {
            continue;
        }
        let val = bb[m - 1 - k];
        let left = aa.lower_bound(&val);
        let right = aa.upper_bound(&val);
        ans[k] = right - left;
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}

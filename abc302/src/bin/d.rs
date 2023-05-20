use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m, d): (usize, usize, i64),
        mut aa: [i64; n],
        mut bb: [i64; m],
    }

    aa.sort_unstable();
    aa.dedup();
    bb.sort_unstable();
    bb.dedup();

    let mut ans = 0;
    for &a in &aa {
        let lower_idx = bb.lower_bound(&(a - d));
        let upper_idx = bb.upper_bound(&(a + d));

        if lower_idx >= upper_idx {
            continue;
        }

        ans = ans.max(a + bb[upper_idx - 1]);
    }

    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

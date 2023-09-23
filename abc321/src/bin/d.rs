use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m, p): (usize, usize, usize),
        mut aa: [usize; n],
        mut bb: [usize; m],
    }

    aa.sort_unstable();
    bb.sort_unstable();

    let mut acc_bb = vec![0];
    for &b in &bb {
        acc_bb.push(acc_bb.last().unwrap() + b);
    }

    let mut ans = 0;
    for &a in &aa {
        let cnt = bb.upper_bound(&p.saturating_sub(a));
        ans += a * cnt + acc_bb[cnt] + p * (m - cnt);
    }
    println!("{}", ans);
}

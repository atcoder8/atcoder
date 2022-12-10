use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, t): (usize, usize),
        aa: [usize; n],
    }

    let sum: usize = aa.iter().sum();

    let mut bb = vec![0];
    for i in 0..aa.len() {
        bb.push(bb.last().unwrap() + aa[i]);
    }

    let t = t % sum;
    let idx = bb.lower_bound(&t);
    let time = if idx == 0 { t } else { t - bb[idx - 1] };

    println!("{} {}", idx, time);
}

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; m],
    }

    for i in 1..=n {
        let pos = aa.lower_bound(&i);
        println!("{}", aa[pos] - i);
    }
}

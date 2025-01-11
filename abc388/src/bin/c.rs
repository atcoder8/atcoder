use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0_usize;
    for &a in &aa {
        ans += n - aa.lower_bound(&(2 * a));
    }
    println!("{}", ans);
}

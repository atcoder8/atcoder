use itertools::enumerate;
use proconio::input;
use superslice::Ext;

const MOD: usize = 10_usize.pow(8);

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let sum = aa.iter().sum::<usize>();

    let mut ans = sum * (n - 1);
    for (i, &a) in enumerate(&aa) {
        let pos = aa[i + 1..].lower_bound(&(MOD - a));
        ans -= MOD * (n - 1 - i - pos);
    }

    println!("{}", ans);
}

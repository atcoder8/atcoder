// unfinished

use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut s: [[usize; m]; n],
    }

    if n == 1 {
        println!("0");
        std::process::exit(0);
    }

    s.iter_mut().for_each(|x| x.sort_unstable());

    let whole = s.iter().flatten().cloned().sorted().collect_vec();

    let mut ans = 0;
    for aa in &s {
        for &a in aa {
            ans += whole.lower_bound(&a) + aa.lower_bound(&a) * (n - 2);
        }
    }

    println!("{}", ans);
}

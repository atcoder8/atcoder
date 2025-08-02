use itertools::enumerate;
use proconio::input;

const MAX: usize = 5 * 10_usize.pow(5);

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0_usize;
    let mut counts = vec![0_usize; MAX];
    for (i, &a) in enumerate(&aa) {
        let j = i + 1;

        if j >= a {
            ans += counts[j - a];
        }

        counts[j + a] += 1;
    }

    println!("{}", ans);
}

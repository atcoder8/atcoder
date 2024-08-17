use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let mut acc = vec![0_usize; n + 1];
    for (i, &a) in enumerate(&aa) {
        acc[i + 1] = (acc[i] + a) % m;
    }

    let mut counts = vec![0_usize; m];
    for &rem in &acc[1..] {
        counts[rem] += 1;
    }

    let mut ans = 0_usize;
    let mut rem = acc[n];
    for (i, &a) in enumerate(&aa) {
        counts[acc[i + 1]] -= 1;
        rem = (rem + a) % m;
        ans += counts[rem];
        counts[rem] += 1;
    }

    println!("{}", ans);
}

use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        qq: [usize; n],
        aa: [usize; n],
        bb: [usize; n],
    }

    let max_a = izip!(&qq, &aa)
        .filter(|&(_, &a)| a != 0)
        .map(|(q, a)| q / a)
        .min()
        .unwrap();

    let mut ans = 0;
    for a in 0..=max_a {
        let b = (0..n)
            .filter(|&i| bb[i] != 0)
            .map(|i| (qq[i] - aa[i] * a) / bb[i])
            .min()
            .unwrap();
        ans = ans.max(a + b);
    }
    println!("{}", ans);
}

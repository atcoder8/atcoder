use itertools::izip;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; m],
        xxx: [[usize; m]; n],
    }

    let mut sums = vec![0; m];
    for xx in &xxx {
        for (sum, x) in izip!(&mut sums, xx) {
            *sum += x;
        }
    }

    let ans = izip!(&aa, &sums).all(|(a, sum)| sum >= a);
    println!("{}", if ans { "Yes" } else { "No" });
}

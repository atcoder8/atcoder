use itertools::izip;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [usize; n],
        mut bb: [usize; n],
        cxv: [(char, Usize1, usize); q],
    }

    let mut sum = izip!(&aa, &bb).map(|(&a, &b)| a.min(b)).sum::<usize>();
    for &(c, x, v) in &cxv {
        let (a, b) = (&mut aa[x], &mut bb[x]);
        let previous = (*a).min(*b);
        if c == 'A' {
            *a = v;
        } else {
            *b = v;
        }
        let current = (*a).min(*b);
        sum = sum - previous + current;
        println!("{sum}");
    }
}

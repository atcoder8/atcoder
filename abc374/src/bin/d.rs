use itertools::{enumerate, iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, s, t): (usize, f64, f64),
        abcd: [((f64, f64), (f64, f64)); n],
    }

    let calc_sum_time = |perm: &[((f64, f64), (f64, f64))], bits: u32| {
        let mut sum_time = 0.0;
        let mut coord = (0.0, 0.0);
        for (i, &(mut from, mut to)) in enumerate(perm) {
            if bits >> i & 1 == 1 {
                std::mem::swap(&mut from, &mut to);
            }

            sum_time += calc_dist(coord, from) / s + calc_dist(from, to) / t;
            coord = to;
        }

        sum_time
    };

    let ans = iproduct!(abcd.iter().cloned().permutations(n), 0..1 << n)
        .map(|(perm, bits)| calc_sum_time(&perm, bits))
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    println!("{}", ans);
}

fn calc_dist(coord1: (f64, f64), coord2: (f64, f64)) -> f64 {
    (coord1.0 - coord2.0).hypot(coord1.1 - coord2.1)
}

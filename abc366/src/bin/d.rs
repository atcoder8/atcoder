use itertools::{iproduct, Itertools};
use ndarray::Array3;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aaa: [[[usize; n]; n]; n],
        q: usize,
    }

    let mut acc = Array3::from_shape_fn((n + 1, n + 1, n + 1), |(x, y, z)| {
        if x == 0 || y == 0 || z == 0 {
            0
        } else {
            aaa[x - 1][y - 1][z - 1]
        }
    });
    for (i, j, k) in iproduct!(0..n, 0..=n, 0..=n) {
        acc[(i + 1, j, k)] += acc[(i, j, k)];
    }
    for (i, j, k) in iproduct!(0..=n, 0..n, 0..=n) {
        acc[(i, j + 1, k)] += acc[(i, j, k)];
    }
    for (i, j, k) in iproduct!(0..=n, 0..=n, 0..n) {
        acc[(i, j, k + 1)] += acc[(i, j, k)];
    }

    let calc_rect_sum = |min_coord: (usize, usize, usize), max_coord: (usize, usize, usize)| {
        let create_coord = |bits: usize| {
            let mut coord = max_coord;
            if bits & 1 == 1 {
                coord.0 = min_coord.0;
            }
            if bits >> 1 & 1 == 1 {
                coord.1 = min_coord.1;
            }
            if bits >> 2 & 1 == 1 {
                coord.2 = min_coord.2;
            }

            coord
        };

        let mut inclusive = 0;
        let mut exclusive = 0;
        for bits in 0..8 {
            let coord = create_coord(bits);
            if bits.count_ones() % 2 == 0 {
                inclusive += acc[coord];
            } else {
                exclusive += acc[coord];
            }
        }

        inclusive - exclusive
    };

    let solve = || {
        input! {
            lx: Usize1,
            rx: usize,
            ly: Usize1,
            ry: usize,
            lz: Usize1,
            rz: usize,
        }

        calc_rect_sum((lx, ly, lz), (rx, ry, rz))
    };

    let ans = (0..q).map(|_| solve()).join("\n");
    println!("{}", ans);
}

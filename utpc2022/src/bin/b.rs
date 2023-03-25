// unfinished

use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }

    let mut ans = 1e9;

    xy.sort_unstable_by_key(|x| x.0);

    let calc_dist = |x_range: (i64, i64), y_range: (i64, i64)| {
        let (min_x, max_x) = x_range;
        let (min_y, max_y) = y_range;

        if min_x <= 0 && 0 <= max_x && min_y <= 0 && 0 <= max_y {
            let min_abs_x = (-min_x).min(max_x);
            let min_abs_y = (-min_y).min(max_y);

            let diff_x = max_x - min_x;
            let diff_y = max_y - min_y;

            (min_abs_x as f64).hypot(min_abs_y as f64) + (diff_x as f64).hypot(diff_y as f64)
        } else if min_x <= 0 && 0 <= max_x {
            let x_dist = (-min_x).min(max_x) + max_x - min_x;
            let y_dist = min_y.abs().max(max_y.abs());

            (x_dist as f64).hypot(y_dist as f64)
        } else if min_y <= 0 && 0 <= max_y {
            let x_dist = min_x.abs().max(max_x.abs());
            let y_dist = (-min_y).min(max_y) + max_y - min_y;

            (x_dist as f64).hypot(y_dist as f64)
        } else {
            let x_dist = min_x.abs().max(max_x.abs());
            let y_dist = min_y.abs().max(max_y.abs());

            (x_dist as f64).hypot(y_dist as f64)
        }
    };

    for left in 0..n {
        let mut ys: BTreeMap<i64, usize> = BTreeMap::new();
        for &(_, y) in &xy {
            *ys.entry(y).or_insert(0) += 1;
        }

        for right in left..=n {
            let (min_x, max_x) = if left == right {
                (0, 0)
            } else {
                (xy[left].0, xy[right - 1].0)
            };

            let (min_y, max_y) = if ys.is_empty() {
                (0, 0)
            } else {
                let min_y = *ys.keys().next().unwrap();
                let max_y = *ys.keys().last().unwrap();

                (min_y, max_y)
            };

            let dist = calc_dist((min_x, max_x), (min_y, max_y));
            if dist < ans {
                ans = dist;
            }

            if right != n {
                let remove_y = xy[right].1;

                *ys.get_mut(&remove_y).unwrap() -= 1;
                if *ys.get(&remove_y).unwrap() == 0 {
                    ys.remove(&remove_y);
                }
            }
        }
    }

    println!("{}", ans);
}

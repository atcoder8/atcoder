use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut max = 0;
    for &(x, _) in &xy {
        max = max.max(xy.iter().filter(|v| v.0 == x).count());
    }
    for (i, j) in (0..n).tuple_combinations() {
        let (x1, y1) = xy[i];
        let (x2, y2) = xy[j];

        if x1 == x2 {
            continue;
        }

        let diff_x = x2 - x1;
        let diff_y = y2 - y1;

        let mut cnt = 2;
        for k in j + 1..n {
            let (x, y) = xy[k];

            if x == x1 {
                continue;
            }

            let dx = x - x1;
            let dy = y - y1;

            cnt += (diff_x * dy == diff_y * dx) as usize;
        }

        max = max.max(cnt);
    }

    let ans = if max > 2 * (n - max) { n - max } else { n / 3 };
    println!("{}", ans);
}

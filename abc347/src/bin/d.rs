use itertools::iproduct;
use ndarray::Array;
use proconio::input;

fn main() {
    match solve() {
        Some((x, y)) => println!("{} {}", x, y),
        None => println!("-1"),
    }
}

fn solve() -> Option<(usize, usize)> {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let mut dp = Array::from_elem((61, a + 1, b + 1), false);
    dp[(0, 0, 0)] = true;
    for d in 0..60 {
        for (prev_cx, prev_cy) in iproduct!(0..=a, 0..=b) {
            if !dp[(d, prev_cx, prev_cy)] {
                continue;
            }

            if c >> d & 1 == 0 {
                dp[(d + 1, prev_cx, prev_cy)] = true;

                if prev_cx < a && prev_cy < b {
                    dp[(d + 1, prev_cx + 1, prev_cy + 1)] = true;
                }
            } else {
                if prev_cx < a {
                    dp[(d + 1, prev_cx + 1, prev_cy)] = true;
                }

                if prev_cy < b {
                    dp[(d + 1, prev_cx, prev_cy + 1)] = true;
                }
            }
        }
    }

    if !dp[(60, a, b)] {
        return None;
    }

    let mut x = 0_usize;
    let mut y = 0_usize;
    let mut cx = a;
    let mut cy = b;
    for d in (0..60).rev() {
        if cx > 0 && cy > 0 && dp[(d, cx - 1, cy - 1)] {
            x += 1 << d;
            y += 1 << d;
            cx -= 1;
            cy -= 1;
        } else if cx > 0 && dp[(d, cx - 1, cy)] {
            x += 1 << d;
            cx -= 1;
        } else if cy > 0 && dp[(d, cx, cy - 1)] {
            y += 1 << d;
            cy -= 1;
        } else {
            assert!(dp[(d, cx, cy)]);
        }
    }

    Some((x, y))
}

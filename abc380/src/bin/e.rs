use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let queries = (0..q).map(|_| Query::read()).collect_vec();

    let mut count_per_color = vec![1_usize; n];
    let mut color_map: BTreeMap<(usize, usize), usize> = (0..n).map(|i| ((i, 1), i)).collect();
    for query in queries {
        match query {
            Query::Paint { start, color } => {
                let (&range, &prev_color) = color_map.range(..(start + 1, 0)).next_back().unwrap();
                color_map.remove(&range);
                count_per_color[prev_color] -= range.1;
                count_per_color[color] += range.1;

                let (mut left, mut len) = range;
                if let Some((&left_range, &left_color)) = color_map.range(..(left, 0)).next_back() {
                    if left_color == color {
                        color_map.remove(&left_range);
                        left = left_range.0;
                        len += left_range.1;
                    }
                }
                if let Some((&right_range, &right_color)) =
                    color_map.range((left + len, 0)..).next()
                {
                    if right_color == color {
                        color_map.remove(&right_range);
                        len += right_range.1;
                    }
                }

                color_map.insert((left, len), color);
            }
            Query::Count { color } => {
                println!("{}", count_per_color[color]);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Paint { start: usize, color: usize },
    Count { color: usize },
}

impl Query {
    fn read() -> Self {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (x, c): (Usize1, Usize1),
            }

            Query::Paint { start: x, color: c }
        } else {
            input! {
                c: Usize1,
            }

            Query::Count { color: c }
        }
    }
}

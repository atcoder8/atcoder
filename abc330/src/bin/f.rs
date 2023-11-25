// unfinished

use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        xy: [(usize, usize); n],
    }

    let mut hor: BTreeMap<usize, usize> = BTreeMap::new();
    let mut ver: BTreeMap<usize, usize> = BTreeMap::new();
    for &(x, y) in &xy {
        *hor.entry(x).or_default() += 1;
        *ver.entry(y).or_default() += 1;
    }

    let mut rem = k;
    while rem != 0 {
        let top = *hor.iter().next().unwrap().0;
        let bottom = *hor.iter().next_back().unwrap().0;
        let x_size = bottom - top;

        let left = *ver.iter().next().unwrap().0;
        let right = *ver.iter().next_back().unwrap().0;
        let y_size = right - left;

        if x_size > y_size {
            let top_num = hor[&top];
            let bottom_num = hor[&bottom];

            if top_num < bottom_num {
                let next_top = *hor.range(top + 1..).next().unwrap().0;
                let reduce_size = (rem / top_num).min(x_size - y_size).min(next_top - top);
                hor.remove(&top);
                if top + reduce_size == next_top {
                    *hor.get_mut(&next_top).unwrap() += top_num;
                } else {
                    hor.insert(top + reduce_size, top_num);
                }

                if rem / top_num < (x_size - y_size).min(next_top - top) {
                    break;
                } else {
                    rem -= top_num * reduce_size;
                }
            } else {
                let next_bottom = *hor.range(..bottom).next_back().unwrap().0;
                let reduce_size = (rem / top_num)
                    .min(x_size - y_size)
                    .min(bottom - next_bottom);
                hor.remove(&bottom);
                if bottom - reduce_size == next_bottom {
                    *hor.get_mut(&next_bottom).unwrap() += bottom_num;
                } else {
                    hor.insert(bottom - reduce_size, bottom_num);
                }

                if rem / top_num < (x_size - y_size).min(bottom - next_bottom) {
                    break;
                } else {
                    rem -= bottom_num * reduce_size;
                }
            }
        } else if x_size < y_size {
            let left_num = ver[&top];
            let right_num = ver[&bottom];

            if left_num < right_num {
                let next_left = *ver.range(top + 1..).next().unwrap().0;
                let reduce_size = (rem / left_num).min(x_size - y_size).min(next_left - top);
                ver.remove(&top);
                if top + reduce_size == next_left {
                    *ver.get_mut(&next_left).unwrap() += left_num;
                } else {
                    ver.insert(top + reduce_size, left_num);
                }

                if rem / left_num < (x_size - y_size).min(next_left - top) {
                    break;
                } else {
                    rem -= left_num * reduce_size;
                }
            } else {
                let next_right = *ver.range(..bottom).next_back().unwrap().0;
                let reduce_size = (rem / left_num)
                    .min(x_size - y_size)
                    .min(bottom - next_right);
                ver.remove(&bottom);
                if bottom - reduce_size == next_right {
                    *ver.get_mut(&next_right).unwrap() += right_num;
                } else {
                    ver.insert(bottom - reduce_size, right_num);
                }

                if rem / left_num < (x_size - y_size).min(bottom - next_right) {
                    break;
                } else {
                    rem -= right_num * reduce_size;
                }
            }
        }
    }

    let top = *hor.iter().next().unwrap().0;
    let bottom = *hor.iter().next_back().unwrap().0;
    let x_size = bottom - top;

    let left = *ver.iter().next().unwrap().0;
    let right = *ver.iter().next_back().unwrap().0;
    let y_size = right - left;

    println!("{}", x_size.max(y_size));
}

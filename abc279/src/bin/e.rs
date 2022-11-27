use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; m],
    }

    let mut full_map = (0..n).collect_vec();
    let mut inv_full_map = (0..n).collect_vec();
    for &a in &aa {
        inv_full_map.swap(full_map[a], full_map[a + 1]);
        full_map.swap(a, a + 1);
    }

    let mut sub_map = (0..n).collect_vec();
    let mut inv_sub_map = (0..n).collect_vec();

    for &a in &aa {
        let ans = if sub_map[a] == 0 {
            inv_full_map[sub_map[a + 1]]
        } else if sub_map[a + 1] == 0 {
            inv_full_map[sub_map[a]]
        } else {
            inv_full_map[0]
        } + 1;

        println!("{}", ans);

        inv_sub_map.swap(sub_map[a], sub_map[a + 1]);
        sub_map.swap(a, a + 1);
    }
}

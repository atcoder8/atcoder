use std::{cmp::Reverse, collections::BinaryHeap};

use im_rc::HashMap;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        rcx: [(Usize1, Usize1, usize); n],
    }

    let rows = rcx.iter().map(|x| x.0).collect_vec();
    let (zip_rows, unzip_rows) = coordinate_compression(&rows);

    let cols = rcx.iter().map(|x| x.1).collect_vec();
    let (zip_cols, unzip_cols) = coordinate_compression(&cols);

    let zip_rcx_map: HashMap<(usize, usize), usize> = (0..n)
        .map(|i| ((zip_rows[i], zip_cols[i]), rcx[i].2))
        .collect();

    let mut hor_sums = vec![0; unzip_rows.len()];
    let mut ver_sums = vec![0; unzip_cols.len()];

    for (&(r, c), &x) in &zip_rcx_map {
        hor_sums[r] += x;
        ver_sums[c] += x;
    }

    let sorted_hor_sums = hor_sums
        .iter()
        .enumerate()
        .map(|(i, &sum)| (sum, i))
        .sorted_by_key(|x| Reverse(x.0))
        .collect_vec();
    let sorted_ver_sums = ver_sums
        .iter()
        .enumerate()
        .map(|(i, &sum)| (sum, i))
        .sorted_by_key(|x| Reverse(x.0))
        .collect_vec();

    let mut heap = BinaryHeap::new();

    for col_idx in 0..sorted_ver_sums.len() {
        heap.push((
            sorted_hor_sums[0].0 + sorted_ver_sums[col_idx].0,
            0,
            col_idx,
        ));
    }

    let mut ans = 0;

    while let Some((cross_sum, row_idx, col_idx)) = heap.pop() {
        if cross_sum <= ans {
            break;
        }

        let row = sorted_hor_sums[row_idx].1;
        let col = sorted_ver_sums[col_idx].1;

        ans = ans.max(cross_sum - zip_rcx_map.get(&(row, col)).unwrap_or(&0));

        if row_idx + 1 < sorted_hor_sums.len() {
            heap.push((
                sorted_hor_sums[row_idx + 1].0 + sorted_ver_sums[col_idx].0,
                row_idx + 1,
                col_idx,
            ));
        }
    }

    println!("{}", ans);
}

/// Performs coordinate compression of `seq`.
///
/// The return value is a tuple of `zip` and `unzip`.
/// `zip` is a list of the number of smallest values in the whole sequence for each element.
/// `unzip` is a list of the values that appear in the number sequence in ascending order.
/// The `i`-th element of the original sequence can be restored by `unzip[zip[i]]`.
pub fn coordinate_compression<T>(seq: &[T]) -> (Vec<usize>, Vec<T>)
where
    T: Clone + Ord,
{
    let mut unzip = seq.to_owned();
    unzip.sort_unstable();
    unzip.dedup();

    let zip: Vec<usize> = seq
        .iter()
        .map(|x| unzip.binary_search(x).unwrap())
        .collect();

    (zip, unzip)
}

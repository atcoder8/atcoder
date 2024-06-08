use itertools::{iproduct, Itertools};
use ndarray::{Array2, Axis};
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut block_per_level = vec![];
    block_per_level.push(Array2::from_elem((1, 1), true));

    for level in 1..=n {
        let prev_block = &block_per_level[level - 1];
        let block_len = 3_usize.pow((level - 1) as u32);
        let mut block = Array2::from_elem((3 * block_len, 3 * block_len), false);
        for (block_row, block_col) in iproduct!(0..3, 0..3) {
            if block_row == 1 && block_col == 1 {
                continue;
            }

            let top = block_len * block_row;
            let left = block_len * block_col;
            block
                .slice_mut(ndarray::s![top..top + block_len, left..left + block_len])
                .assign(prev_block);
        }

        block_per_level.push(block);
    }

    let ans = block_per_level[n]
        .axis_iter(Axis(0))
        .map(|line| {
            line.iter()
                .map(|&black| (if black { '#' } else { '.' }))
                .join("")
        })
        .join("\n");
    println!("{}", ans);
}

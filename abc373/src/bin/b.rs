use itertools::{enumerate, Itertools};
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    }

    let mut positions = vec![0_usize; 26];
    for (i, b) in enumerate(s) {
        positions[(b - b'A') as usize] = i;
    }

    let ans = positions
        .iter()
        .tuple_windows()
        .map(|(&pos1, &pos2)| pos1.abs_diff(pos2))
        .sum::<usize>();
    println!("{}", ans);
}

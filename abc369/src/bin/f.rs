use itertools::{chain, enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        mut rc: [(usize, usize); n],
    }

    rc.sort_unstable();

    let mut dp: Vec<Vec<(usize, usize)>> = vec![];
    for (i, &(_, c)) in enumerate(&rc) {
        let pos = dp.partition_point(|v| v.last().unwrap().0 <= c);
        if pos == dp.len() {
            dp.push(vec![(c, i)]);
        } else {
            dp[pos].push((c, i));
        }
    }

    let mut path = vec![(h, w)];
    let mut cur = n;
    for v in dp.iter().rev() {
        let (_, time) = *v.iter().rev().find(|&&(_, idx)| idx < cur).unwrap();
        path.push(rc[time]);
        cur = time;
    }
    path.push((1, 1));
    path.reverse();

    let directions = path
        .iter()
        .tuple_windows()
        .flat_map(|((r1, c1), (r2, c2))| {
            chain!(
                std::iter::repeat('D').take(r2 - r1),
                std::iter::repeat('R').take(c2 - c1),
            )
        })
        .collect::<String>();
    println!("{}\n{}", dp.len(), directions);
}

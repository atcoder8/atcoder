use itertools::Itertools;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<f64> {
    input! {
        n: usize,
        xh: [(usize, usize); n],
    }

    if xh
        .iter()
        .tuple_windows()
        .all(|(&(x1, y1), &(x2, y2))| y1 * x2 < x1 * y2)
    {
        return None;
    }

    let calc_required_height = |coord1: (usize, usize), coord2: (usize, usize)| {
        let (x1, y1) = coord1;
        let (x2, y2) = coord2;
        let numer = (x2 * y1).saturating_sub(x1 * y2);
        let denom = x2 - x1;
        numer as f64 / denom as f64
    };

    let max_required_height = xh
        .iter()
        .tuple_windows()
        .map(|(&coord1, &coord2)| calc_required_height(coord1, coord2))
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    Some(max_required_height)
}

use itertools::iproduct;
use proconio::input;

const HEIGHT: usize = 2;
const WIDTH: usize = 4;
const GRID: [[usize; WIDTH]; HEIGHT] = [[2, 1, 0, 1], [1, 2, 1, 0]];
const BLACK_AREA: usize = 8;
const OFFSET: i64 = 10_i64.pow(9);

fn main() {
    input! {
        (a, b, c, d): (i64, i64, i64, i64),
    }

    let a = (a + OFFSET) as usize;
    let b = (b + OFFSET) as usize;
    let c = (c + OFFSET) as usize;
    let d = (d + OFFSET) as usize;

    let inclusion = calc_rect(b, a) + calc_rect(d, c);
    let exclusion = calc_rect(d, a) + calc_rect(b, c);
    let ans = inclusion - exclusion;
    println!("{}", ans);
}

fn calc_rect(h: usize, w: usize) -> usize {
    let (qh, rh) = (h / HEIGHT, h % HEIGHT);
    let (qw, rw) = (w / WIDTH, w % WIDTH);

    let completed_area = BLACK_AREA * qh * qw;
    let hor_area = iproduct!(0..rh, 0..WIDTH)
        .map(|(row, col)| GRID[row][col])
        .sum::<usize>()
        * qw;
    let ver_area = iproduct!(0..HEIGHT, 0..rw)
        .map(|(row, col)| GRID[row][col])
        .sum::<usize>()
        * qh;
    let rem_area = iproduct!(0..rh, 0..rw)
        .map(|(row, col)| GRID[row][col])
        .sum::<usize>();

    completed_area + hor_area + ver_area + rem_area
}

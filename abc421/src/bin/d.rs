use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        (rt, ct, ra, ca): (i64, i64, i64, i64),
        (_n, m, l): (usize, usize, usize),
        sa: [(char, i64); m],
        tb: [(char, i64); l],
    }

    let mut coord1 = [rt, ct];
    let mut coord2 = [ra, ca];
    let mut queue1: VecDeque<StraightMove> =
        sa.iter().map(|&(s, a)| StraightMove::new(s, a)).collect();
    let mut queue2: VecDeque<StraightMove> =
        tb.iter().map(|&(t, b)| StraightMove::new(t, b)).collect();

    let mut cross_times = 0_i64;
    while let (Some(move1), Some(move2)) = (queue1.pop_front(), queue2.pop_front()) {
        let StraightMove {
            direction: dir1,
            distance: dist1,
        } = move1;
        let StraightMove {
            direction: dir2,
            distance: dist2,
        } = move2;

        let min_dist = dist1.min(dist2);

        cross_times += calc_cross_times(coord1, coord2, dir1, dir2, min_dist);
        coord1 = moved(coord1, dir1, min_dist);
        coord2 = moved(coord2, dir2, min_dist);
        if dist1 > min_dist {
            queue1.push_front(StraightMove {
                direction: dir1,
                distance: dist1 - min_dist,
            });
        }
        if dist2 > min_dist {
            queue2.push_front(StraightMove {
                direction: dir2,
                distance: dist2 - min_dist,
            });
        }
    }
    println!("{}", cross_times);
}

fn dir_to_diffs(dir: char) -> [i64; 2] {
    match dir {
        'U' => [-1, 0],
        'D' => [1, 0],
        'L' => [0, -1],
        'R' => [0, 1],
        _ => panic!(),
    }
}

fn calc_cross_times(coord1: [i64; 2], coord2: [i64; 2], dir1: char, dir2: char, dist: i64) -> i64 {
    if dir1 == dir2 {
        return if coord1 == coord2 { dist } else { 0 };
    }

    let [x1, y1] = coord1;
    let [x2, y2] = coord2;
    let [dx, dy] = [x2 - x1, y2 - y1];

    let [sx1, sy1] = dir_to_diffs(dir1);
    let [sx2, sy2] = dir_to_diffs(dir2);
    let [sx, sy] = [sx1 - sx2, sy1 - sy2];

    if sx == 0 {
        if dx != 0 || dy % sy != 0 {
            return 0;
        }

        let t = dy / sy;
        return (t > 0 && t <= dist) as i64;
    }

    if sy == 0 {
        if dy != 0 || dx % sx != 0 {
            return 0;
        }

        let t = dx / sx;
        return (t > 0 && t <= dist) as i64;
    }

    if dx % sx != 0 || dy % sy != 0 {
        return 0;
    }

    let tx = dx / sx;
    let ty = dy / sy;

    (tx > 0 && tx <= dist && tx == ty) as i64
}

fn moved(coord: [i64; 2], dir: char, dist: i64) -> [i64; 2] {
    let mut moved_coord = coord;
    match dir {
        'U' => moved_coord[0] -= dist,
        'D' => moved_coord[0] += dist,
        'L' => moved_coord[1] -= dist,
        'R' => moved_coord[1] += dist,
        _ => panic!(),
    }

    moved_coord
}

#[derive(Debug, Clone, Copy)]
struct StraightMove {
    direction: char,
    distance: i64,
}

impl StraightMove {
    fn new(direction: char, distance: i64) -> Self {
        StraightMove {
            direction,
            distance,
        }
    }
}

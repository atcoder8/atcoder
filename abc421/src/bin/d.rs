use proconio::input;

fn main() {
    input! {
        (mut coord1, mut coord2): ((i64, i64), (i64, i64)),
        (_n, m, l): (i64, usize, usize),
        mut moves1: [(char, i64); m],
        mut moves2: [(char, i64); l],
    }

    moves1.reverse();
    moves2.reverse();

    let mut num_crosses = 0_i64;
    while let (Some(move1), Some(move2)) = (moves1.pop(), moves2.pop()) {
        let (dir1, num_moves_1) = move1;
        let (dir2, num_moves_2) = move2;

        let min_num_moves = num_moves_1.min(num_moves_2);

        let rem_num_moves_1 = num_moves_1 - min_num_moves;
        if rem_num_moves_1 > 0 {
            moves1.push((dir1, rem_num_moves_1));
        }
        let rem_num_moves_2 = num_moves_2 - min_num_moves;
        if rem_num_moves_2 > 0 {
            moves2.push((dir2, rem_num_moves_2));
        }

        let dist = calc_manhattan_dist(coord1, coord2);
        if coord1 == coord2 {
            num_crosses += if dir1 == dir2 { min_num_moves } else { 0 };
        } else if dist % 2 == 0 && dist / 2 <= min_num_moves {
            let mid_coord_1 = calc_moved_coord(coord1, dir1, dist / 2);
            let mid_coord_2 = calc_moved_coord(coord2, dir2, dist / 2);
            num_crosses += (mid_coord_1 == mid_coord_2) as i64;
        }

        coord1 = calc_moved_coord(coord1, dir1, min_num_moves);
        coord2 = calc_moved_coord(coord2, dir2, min_num_moves);
    }

    println!("{}", num_crosses);
}

fn calc_manhattan_dist(coord1: (i64, i64), coord2: (i64, i64)) -> i64 {
    let (r1, c1) = coord1;
    let (r2, c2) = coord2;
    (r1.abs_diff(r2) + c1.abs_diff(c2)) as i64
}

fn calc_moved_coord(coord: (i64, i64), dir: char, num_moves: i64) -> (i64, i64) {
    let mut moved_coord = coord;
    match dir {
        'U' => moved_coord.0 -= num_moves,
        'D' => moved_coord.0 += num_moves,
        'L' => moved_coord.1 -= num_moves,
        'R' => moved_coord.1 += num_moves,
        _ => panic!(),
    }
    moved_coord
}

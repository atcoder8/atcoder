use itertools::iproduct;
use proconio::{input, marker::Chars};

type Coord = (usize, usize);

fn main() {
    input! {
        (h, w, d): (usize, usize, usize),
        sss: [Chars; h],
    }

    let count_score = |humidifier_coords: [Coord; 2]| {
        let mut score = 0_usize;
        for coord in iproduct!(0..h, 0..w) {
            let (row, col) = coord;
            if sss[row][col] == '.' {
                score += humidifier_coords
                    .iter()
                    .any(|&humidifier_coord| calc_dist(humidifier_coord, coord) <= d)
                    as usize;
            }
        }

        score
    };

    let mut max_score = 0_usize;
    let coords = iproduct!(0..h, 0..w);
    for humidifier_coords in iproduct!(coords.clone(), coords) {
        let ((row1, col1), (row2, col2)) = humidifier_coords;
        if sss[row1][col1] == '.' && sss[row2][col2] == '.' {
            max_score = max_score.max(count_score([humidifier_coords.0, humidifier_coords.1]));
        }
    }
    println!("{}", max_score);
}

fn calc_dist(coord1: Coord, coord2: Coord) -> usize {
    coord1.0.abs_diff(coord2.0) + coord1.1.abs_diff(coord2.1)
}

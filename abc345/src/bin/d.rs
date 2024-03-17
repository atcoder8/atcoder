use itertools::{iproduct, Itertools};
use ndarray::Array2;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, h, w): (usize, usize, usize),
        ab: [(usize, usize); n],
    }

    let cover_grid = |grid: &mut Array2<bool>, pos: (usize, usize), tile: (usize, usize)| {
        if pos.0 + tile.0 > h || pos.1 + tile.1 > w {
            return false;
        }

        for shift in iproduct!(0..tile.0, 0..tile.1) {
            let coord = (pos.0 + shift.0, pos.1 + shift.1);

            if grid[coord] {
                return false;
            }

            grid[coord] = true;
        }

        true
    };

    let is_ok = |tiles: &[(usize, usize)]| -> bool {
        let mut grid = Array2::from_elem((h, w), false);
        let mut pos_iter = iproduct!(0..h, 0..w);

        for &tile in tiles {
            let pos = pos_iter.find(|&pos| !grid[pos]).unwrap();

            if !cover_grid(&mut grid, pos, tile) {
                return false;
            }
        }

        grid.iter().all(|&covered| covered)
    };

    for tile_num in 1..=n {
        for tiles in ab.iter().cloned().permutations(tile_num) {
            for bit in 0..1 << tile_num {
                let mut rotated_tiles = tiles.clone();
                for i in 0..tile_num {
                    if bit >> i & 1 == 1 {
                        let (a, b) = rotated_tiles[i];
                        rotated_tiles[i] = (b, a);
                    }
                }

                if is_ok(&rotated_tiles) {
                    return true;
                }
            }
        }
    }

    false
}

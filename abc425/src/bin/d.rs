use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

use crate::grid_shape::grid_shape::GridShape2D;

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let grid_shape = GridShape2D::new(h, w);

    let mut grid = Array2::from_shape_fn((h, w), |(row, col)| sss[row][col] == '#');

    let is_ok = |grid: &Array2<bool>, coord: (usize, usize)| {
        !grid[coord]
            && grid_shape
                .adjacent_by_edge(coord)
                .filter(|&coord| grid[coord])
                .count()
                == 1
    };

    let mut paint_coord = iproduct!(0..h, 0..w).collect_vec();
    paint_coord.retain(|&coord| is_ok(&grid, coord));

    while !paint_coord.is_empty() {
        for &coord in &paint_coord {
            grid[coord] = true;
        }

        paint_coord = paint_coord
            .iter()
            .flat_map(|&coord| grid_shape.adjacent_by_edge(coord))
            .collect_vec();
        paint_coord.sort_unstable();
        paint_coord.dedup();
        paint_coord.retain(|&coord| is_ok(&grid, coord));
    }

    let num_black_squares = grid.iter().filter(|&&color| color).count();
    println!("{num_black_squares}");
}

pub mod grid_shape {
    pub mod grid_shape {
        //! グリッドの形状を扱うモジュールです。

        /// 2次元のグリッドの形状を扱う構造体です。
        #[derive(Debug, Clone, Copy)]
        pub struct GridShape2D {
            /// 高さ(行数)
            pub height: usize,

            /// 幅(列数)
            pub width: usize,
        }

        impl GridShape2D {
            /// 4近傍の座標の相対位置です。
            const DIFFS4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

            /// 8近傍の座標の相対位置です。
            const DIFFS8: [(usize, usize); 8] = [
                (!0, !0),
                (!0, 0),
                (!0, 1),
                (0, !0),
                (0, 1),
                (1, !0),
                (1, 0),
                (1, 1),
            ];

            /// 高さ(行数)と幅(列数)を指定してインスタンスを生成します。
            pub fn new(height: usize, width: usize) -> Self {
                Self { height, width }
            }

            /// グリッドの形状を指定してインスタンスを生成します。
            pub fn from_shape(shape: (usize, usize)) -> Self {
                Self {
                    height: shape.0,
                    width: shape.1,
                }
            }

            /// グリッドの形状をタプルで返します。
            pub fn shape(&self) -> (usize, usize) {
                (self.height, self.width)
            }

            /// グリッド全体の面積を返します。
            pub fn area(&self) -> usize {
                self.height * self.width
            }

            /// 座標がグリッドに含まれている場合のみ`true`を返します。
            pub fn in_range(&self, coord: (usize, usize)) -> bool {
                coord.0 < self.height && coord.1 < self.width
            }

            /// グリッドに含まれる近傍の座標を返します。
            pub fn adjacent_coordinates<'a, Diffs>(
                &'a self,
                coord: (usize, usize),
                diffs: Diffs,
            ) -> impl Iterator<Item = (usize, usize)> + 'a
            where
                Diffs: IntoIterator<Item = (usize, usize)> + 'a,
            {
                diffs.into_iter().filter_map(move |diff| {
                    let adj_coord = (coord.0.wrapping_add(diff.0), coord.1.wrapping_add(diff.1));
                    if self.in_range(adj_coord) {
                        Some(adj_coord)
                    } else {
                        None
                    }
                })
            }

            /// グリッドに含まれる4近傍の座標を返します。
            pub fn adjacent_by_edge(
                &self,
                coord: (usize, usize),
            ) -> impl Iterator<Item = (usize, usize)> + '_ {
                self.adjacent_coordinates(coord, Self::DIFFS4)
            }

            /// グリッドに含まれる8近傍の座標を返します。
            pub fn adjacent_by_corner(
                &self,
                coord: (usize, usize),
            ) -> impl Iterator<Item = (usize, usize)> + '_ {
                self.adjacent_coordinates(coord, Self::DIFFS8)
            }
        }
    }
}

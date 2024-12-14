use std::{cmp::Reverse, collections::BinaryHeap};

use grid_shape::GridShape2D;
use ndarray::prelude::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w, x): (usize, usize, usize),
        init_coord: (Usize1, Usize1),
        sss: [[usize; w]; h],
    }

    let grid_shape = GridShape2D::new(h, w);

    let grid = Array2::from_shape_fn((h, w), |(row, col)| sss[row][col]);

    let mut score = grid[init_coord];
    let mut heap: BinaryHeap<(Reverse<usize>, (usize, usize))> = grid_shape
        .adjacent_by_edge(init_coord)
        .map(|coord| (Reverse(grid[coord]), coord))
        .collect();
    let mut visited = Array2::from_elem((h, w), false);
    visited[init_coord] = true;
    while let Some((Reverse(weight), coord)) = heap.pop() {
        if visited[coord] {
            continue;
        }

        visited[coord] = true;

        if weight >= (score + x - 1) / x {
            break;
        }

        score += weight;

        heap.extend(
            grid_shape
                .adjacent_by_edge(coord)
                .map(|adj_coord| (Reverse(grid[adj_coord]), adj_coord)),
        );
    }

    println!("{}", score);
}

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

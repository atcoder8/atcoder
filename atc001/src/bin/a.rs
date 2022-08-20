use std::process;

use rectangular_grid::RectGrid2Dim;

fn main() {
    let (h, w) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut ccc: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        ccc.push(line.trim().chars().collect());
    }

    let mut s = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if ccc[i][j] == 's' {
                s = (i, j);
            }
        }
    }

    let rect_grid = RectGrid2Dim::new((h, w));

    let mut stack = vec![s];

    let mut used = vec![vec![false; w]; h];
    used[s.0][s.1] = true;

    while let Some(curr_coord) = stack.pop() {
        for (next_x, next_y) in rect_grid.neighborhood_coordinates_by_edge(curr_coord) {
            if ccc[next_x][next_y] == 'g' {
                println!("Yes");
                process::exit(0);
            }

            if !used[next_x][next_y] && ccc[next_x][next_y] == '.' {
                used[next_x][next_y] = true;
                stack.push((next_x, next_y));
            }
        }
    }

    println!("No");
}

pub mod rectangular_grid {
    //! This module supports the creation of a list of neighborhood coordinates
    //! for a point on a rectangular grid.

    /// Supports 2D rectangular grid.
    pub struct RectGrid2Dim(usize, usize);

    impl RectGrid2Dim {
        /// Creates a 2D rectangular grid.
        ///
        /// # Arguments
        ///
        /// * `rect_size` - Size of rectangle
        pub fn new(rect_size: (usize, usize)) -> Self {
            let (h, w) = rect_size;

            assert!(
                h >= 1 && w >= 1,
                "The length of each side must be greater than or equal to 1."
            );

            Self(h, w)
        }

        /// It creates a list of coordinates that share an edge with `target_coord`.
        ///
        /// The list is ordered lexicographically.
        ///
        /// The list contains up to 4 elements.
        ///
        /// # Example
        ///
        /// ```
        /// use rectangular_grid::RectGrid2Dim;
        ///
        /// let rect_grid = RectGrid2Dim::new((3, 3));
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_edge((1, 1)),
        ///            vec![(0, 1), (1, 0), (1, 2), (2, 1)]);
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_edge((0, 0)),
        ///            vec![(0, 1), (1, 0)]);
        /// ```
        ///
        /// # Panics
        ///
        /// This function panics if `target_coord` is outside the range of the rectangle.
        pub fn neighborhood_coordinates_by_edge(
            &self,
            target_coord: (usize, usize),
        ) -> Vec<(usize, usize)> {
            let &RectGrid2Dim(h, w) = self;
            let (x, y) = target_coord;

            assert!(x < h && y < w, "Out of range coordinate was specified.");

            let mut nei_coords = vec![];

            if x > 0 {
                nei_coords.push((x - 1, y));
            }
            if y > 0 {
                nei_coords.push((x, y - 1));
            }
            if y < w - 1 {
                nei_coords.push((x, y + 1));
            }
            if x < h - 1 {
                nei_coords.push((x + 1, y));
            }

            nei_coords
        }

        /// It create a list of coordinates that share a corner with `target_coord`.
        ///
        /// The list is ordered lexicographically.
        ///
        /// The list contains up to 8 elements.
        ///
        /// # Example
        ///
        /// ```
        /// use rectangular_grid::RectGrid2Dim;
        ///
        /// let rect_grid = RectGrid2Dim::new((3, 3));
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_corner((1, 1)),
        ///            vec![(0, 0), (0, 1), (0, 2), (1, 0),
        ///                 (1, 2), (2, 0), (2, 1), (2, 2)]);
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_corner((0, 0)),
        ///            vec![(0, 1), (1, 0), (1, 1)]);
        /// ```
        ///
        /// # Panics
        ///
        /// This function panics if `target_coord` is outside the range of the rectangle.
        pub fn neighborhood_coordinates_by_corner(
            &self,
            target_coord: (usize, usize),
        ) -> Vec<(usize, usize)> {
            let &RectGrid2Dim(h, w) = self;
            let (x, y) = target_coord;

            assert!(x < h && y < w, "Out of range coordinate was specified.");

            let mut nei_coords = vec![];

            for diffs in 0..9 {
                let (diff_x, diff_y) = (diffs / 3 % 3, diffs % 3);

                if (diff_x, diff_y) != (1, 1)
                    && x + diff_x > 0
                    && x + diff_x <= h
                    && y + diff_y > 0
                    && y + diff_y <= w
                {
                    nei_coords.push((x + diff_x - 1, y + diff_y - 1));
                }
            }

            nei_coords
        }
    }

    /// Supports 3D rectangular grid.
    pub struct RectGrid3Dim(usize, usize, usize);

    impl RectGrid3Dim {
        /// Creates a 3D rectangular grid.
        ///
        /// # Arguments
        ///
        /// * `rect_size` - Size of rectangle
        pub fn new(rect_size: (usize, usize, usize)) -> Self {
            let (h, w, d) = rect_size;

            assert!(
                h >= 1 && w >= 1 && d >= 1,
                "The length of each side must be greater than or equal to 1."
            );

            Self(h, w, d)
        }

        /// It create a list of coordinates that share a surface with `target_coord`.
        ///
        /// The list is ordered lexicographically.
        ///
        /// The list contains up to 6 elements.
        ///
        /// # Example
        ///
        /// ```
        /// use rectangular_grid::RectGrid3Dim;
        ///
        /// let rect_grid = RectGrid3Dim::new((3, 3, 3));
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_surface((1, 1, 1)),
        ///            vec![(0, 1, 1), (1, 0, 1), (1, 1, 0),
        ///                 (1, 1, 2), (1, 2, 1), (2, 1, 1)]);
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_surface((0, 0, 0)),
        ///            vec![(0, 0, 1), (0, 1, 0), (1, 0, 0)]);
        /// ```
        ///
        /// # Panics
        ///
        /// This function panics if `target_coord` is outside the range of the rectangle.
        pub fn neighborhood_coordinates_by_surface(
            &self,
            target_coord: (usize, usize, usize),
        ) -> Vec<(usize, usize, usize)> {
            let &RectGrid3Dim(h, w, d) = self;
            let (x, y, z) = target_coord;

            assert!(
                x < h && y < w && z < d,
                "Out of range coordinate was specified."
            );

            let mut nei_coords = vec![];

            if x > 0 {
                nei_coords.push((x - 1, y, z));
            }
            if y > 0 {
                nei_coords.push((x, y - 1, z));
            }
            if z > 0 {
                nei_coords.push((x, y, z - 1));
            }
            if z < d - 1 {
                nei_coords.push((x, y, z + 1));
            }
            if y < w - 1 {
                nei_coords.push((x, y + 1, z));
            }
            if x < h - 1 {
                nei_coords.push((x + 1, y, z));
            }

            nei_coords
        }

        /// It creates a list of coordinates that share a corner with `target_coord`.
        ///
        /// The list is ordered lexicographically.
        ///
        /// The list contains up to 26 elements.
        ///
        /// # Example
        ///
        /// ```
        /// use rectangular_grid::RectGrid3Dim;
        ///
        /// let rect_grid = RectGrid3Dim::new((3, 3, 3));
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_corner((1, 1, 1)),
        ///            vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 1, 0),
        ///                 (0, 1, 1), (0, 1, 2), (0, 2, 0), (0, 2, 1),
        ///                 (0, 2, 2), (1, 0, 0), (1, 0, 1), (1, 0, 2),
        ///                 (1, 1, 0), (1, 1, 2), (1, 2, 0), (1, 2, 1),
        ///                 (1, 2, 2), (2, 0, 0), (2, 0, 1), (2, 0, 2),
        ///                 (2, 1, 0), (2, 1, 1), (2, 1, 2), (2, 2, 0),
        ///                 (2, 2, 1), (2, 2, 2)]);
        /// assert_eq!(rect_grid.neighborhood_coordinates_by_corner((0, 0, 0)),
        ///            vec![(0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0),
        ///                 (1, 0, 1), (1, 1, 0), (1, 1, 1)]);
        /// ```
        ///
        /// # Panics
        ///
        /// This function panics if `target_coord` is outside the range of the rectangle.
        pub fn neighborhood_coordinates_by_corner(
            &self,
            target_coord: (usize, usize, usize),
        ) -> Vec<(usize, usize, usize)> {
            let &RectGrid3Dim(h, w, d) = self;
            let (x, y, z) = target_coord;

            assert!(
                x < h && y < w && z < d,
                "Out of range coordinate was specified."
            );

            let mut nei_coords = vec![];

            for diffs in 0..27 {
                let (diff_x, diff_y, diff_z) = (diffs / 9 % 3, diffs / 3 % 3, diffs % 3);

                if (diff_x, diff_y, diff_z) != (1, 1, 1)
                    && x + diff_x > 0
                    && x + diff_x <= h
                    && y + diff_y > 0
                    && y + diff_y <= w
                    && z + diff_z > 0
                    && z + diff_z <= d
                {
                    nei_coords.push((x + diff_x - 1, y + diff_y - 1, z + diff_z - 1));
                }
            }

            nei_coords
        }
    }

    /// Supports multi-dimensional rectangular grid.
    pub struct RectGridMultiDim(Vec<usize>);

    impl RectGridMultiDim {
        /// Creates a multi-dimensional rectangular grid.
        ///
        /// # Arguments
        ///
        /// * `rect_size` - Size of rectangle
        pub fn new(rect_size: Vec<usize>) -> Self {
            assert_ne!(
                rect_size.len(),
                0,
                "The number of dimensions must be at least 1."
            );

            for &length in rect_size.iter() {
                assert!(length >= 1, "The length of each side must be at least 1.");
            }

            Self(rect_size)
        }

        /// It creates a list of coordinates that share a surface with `target_coord`.
        ///
        /// The list is ordered lexicographically.
        ///
        /// The list contains up to `2n` elements (`n` is the number of dimensions).
        ///
        /// # Panics
        ///
        /// This function panics if
        /// * The number of dimensions of `target_coord` is
        ///   different from the number of dimensions of the rectangle
        /// * `target_coord` is outside the range of the rectangle
        pub fn neighborhood_coordinates_by_surface(
            &self,
            target_coord: &Vec<usize>,
        ) -> Vec<Vec<usize>> {
            let RectGridMultiDim(rect_size) = self;

            assert_eq!(
                rect_size.len(),
                target_coord.len(),
                "The number of dimensions do not match."
            );

            for i in 0..rect_size.len() {
                assert!(
                    target_coord[i] < rect_size[i],
                    "Out of range coordinate was specified."
                );
            }

            let mut nei_coords = vec![];

            for (i, &x) in target_coord.iter().enumerate() {
                if x > 0 {
                    let mut nei_coord = target_coord.clone();
                    nei_coord[i] = target_coord[i] - 1;
                    nei_coords.push(nei_coord);
                }
            }

            for (i, &x) in target_coord.iter().enumerate().rev() {
                if x < rect_size[i] - 1 {
                    let mut nei_coord = target_coord.clone();
                    nei_coord[i] = target_coord[i] + 1;
                    nei_coords.push(nei_coord);
                }
            }

            nei_coords
        }

        /// It creates a list of coordinates that share a corner with `target_coord`.
        ///
        /// The list is ordered lexicographically.
        ///
        /// The list contains up to `3^n - 1` elements (`n` is the number of dimensions).
        ///
        /// # Panics
        ///
        /// This function panics if
        /// * The number of dimensions of the rectangle is greater than or equal to 41
        /// * The number of dimensions of `target_coord` is
        ///   different from the number of dimensions of the rectangle
        /// * `target_coord` is outside the range of the rectangle
        pub fn neighborhood_coordinates_by_corner(
            &self,
            target_coord: &Vec<usize>,
        ) -> Vec<Vec<usize>> {
            let RectGridMultiDim(rect_size) = self;

            let n = rect_size.len();

            assert!(
                rect_size.len() <= 40,
                "The number of dimensions of the rectangle is too large."
            );

            assert_eq!(
                target_coord.len(),
                n,
                "The number of dimensions do not match."
            );

            for i in 0..n {
                assert!(
                    target_coord[i] < rect_size[i],
                    "Out of range coordinate was specified."
                );
            }

            let mut nei_coords = vec![];

            for diffs in 0..3_usize.pow(n as u32) {
                if let Some(nei_coord) = self.create_nei_coord(target_coord, diffs) {
                    nei_coords.push(nei_coord);
                }
            }

            nei_coords
        }

        fn create_nei_coord(
            &self,
            target_coord: &Vec<usize>,
            mut diffs: usize,
        ) -> Option<Vec<usize>> {
            let RectGridMultiDim(rect_size) = self;

            let mut nei_coord = vec![0; target_coord.len()];

            for i in (0..rect_size.len()).rev() {
                let diff = diffs % 3;
                diffs /= 3;

                if target_coord[i] + diff > 0 && target_coord[i] + diff <= rect_size[i] {
                    nei_coord[i] = target_coord[i] + diff - 1;
                } else {
                    return None;
                }
            }

            if nei_coord != *target_coord {
                Some(nei_coord)
            } else {
                None
            }
        }
    }
}

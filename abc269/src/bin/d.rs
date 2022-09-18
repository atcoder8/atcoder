use std::collections::HashSet;

use proconio::input;

use crate::atcoder8_library::union_find::UnionFind;

const JACK_UP: i64 = 1010;

const GRID_SIDE_LEN: usize = 2100;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let xy: Vec<(usize, usize)> = xy
        .into_iter()
        .map(|(x, y)| ((x + JACK_UP) as usize, (y + JACK_UP) as usize))
        .collect();

    let mut grid = vec![vec![false; GRID_SIDE_LEN]; GRID_SIDE_LEN];
    for &(x, y) in &xy {
        grid[x][y] = true;
    }

    let mut uf = UnionFind::new(GRID_SIDE_LEN * GRID_SIDE_LEN);

    for &(x, y) in &xy {
        if !grid[x][y] {
            continue;
        }

        let check_coords = vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y),
            (x + 1, y + 1),
        ];

        for &(check_x, check_y) in &check_coords {
            if grid[check_x][check_y] {
                uf.merge(x * GRID_SIDE_LEN + y, check_x * GRID_SIDE_LEN + check_y);
            }
        }
    }

    let mut leaders = HashSet::new();
    for &(x, y) in &xy {
        leaders.insert(uf.leader(x * GRID_SIDE_LEN + y));
    }

    println!("{}", leaders.len());
}

pub mod atcoder8_library {
    pub mod union_find {
        //! Union-Find processes the following queries for an edgeless graph in `O(α(n))` amortized time.
        //! * Add an undirected edge.
        //! * Deciding whether given two vertices are in the same connected component
        //!
        //! When a method is called, route compression is performed as appropriate.

        use std::collections::HashMap;

        /// Union-Find processes the following queries for an edgeless graph in `O(α(n))` amortized time.
        /// * Add an undirected edge.
        /// * Deciding whether given two vertices are in the same connected component
        ///
        /// When a method is called, route compression is performed as appropriate.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(0, 1);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(1, 2);
        /// assert_eq!(uf.same(0, 2), true);
        /// ```
        pub struct UnionFind {
            /// For each element, one of the following is stored.
            /// * Size of the connected component to which it belongs; It is expressed by a negative number
            /// (if it is representative of a connected component)
            /// * Index of the element that is its own parent (otherwise)
            parent_or_size: Vec<i32>,

            /// Number of connected components.
            group_num: usize,
        }

        impl UnionFind {
            /// Creates an undirected graph with `n` vertices and 0 edges.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.same(0, 2), true);
            /// ```
            pub fn new(n: usize) -> Self {
                UnionFind {
                    parent_or_size: vec![-1; n],
                    group_num: n,
                }
            }

            /// Returns the representative of the connected component that contains the vertex `a`.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// uf.merge(1, 2);
            /// assert_eq!(uf.leader(0), 0);
            /// assert_eq!(uf.leader(1), uf.leader(2));
            /// ```
            pub fn leader(&mut self, a: usize) -> usize {
                // Path from A to just before the representative
                // of the connected component to which A belongs
                // (If the representative is A, then it is empty)
                let mut path = vec![];

                // Variable representing the current vertex (initialized with a)
                let mut curr = a;

                // Loop until the vertex indicated by curr becomes the parent.
                while self.parent_or_size[curr] >= 0 {
                    // Add curr to the path.
                    path.push(curr);
                    // Move to parent vertex.
                    curr = self.parent_or_size[curr] as usize;
                }

                // Set the parent of every vertex in the path to representative of the connected component.
                path.iter()
                    .for_each(|&x| self.parent_or_size[x] = curr as i32);

                // Return a representative of the connected component.
                curr
            }

            /// Returns whether the vertices `a` and `b` are in the same connected component.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.same(0, 2), true);
            /// ```
            pub fn same(&mut self, a: usize, b: usize) -> bool {
                self.leader(a) == self.leader(b)
            }

            /// Adds an edge between vertex `a` and vertex `b`.
            /// Returns true if the connected component to which a belongs and that of b are newly combined.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.same(0, 2), true);
            /// ```
            pub fn merge(&mut self, a: usize, b: usize) -> bool {
                // Representative of the connected component that contains the vertex a
                let mut leader_a = self.leader(a);
                // Representative of the connected component that contains the vertex b
                let mut leader_b = self.leader(b);

                // If a and b belong to the same connected component, return false without processing.
                if leader_a == leader_b {
                    return false;
                }

                // If the size of the connected component to which a belongs is
                // smaller than that of b, swap a and b.
                if -self.parent_or_size[leader_a] < -self.parent_or_size[leader_b] {
                    std::mem::swap(&mut leader_a, &mut leader_b);
                }

                // Add that of b to the number of elements of the connected component to which a belongs.
                self.parent_or_size[leader_a] += self.parent_or_size[leader_b];

                // Set the parent of the representative of the connected component to which b belongs
                // to the representative of the connected component to which a belongs.
                self.parent_or_size[leader_b] = leader_a as i32;

                // Decrease the number of connected components by one.
                self.group_num -= 1;

                // Return true because the connected component is newly combined.
                true
            }

            /// Returns a list of connected components.
            ///
            /// Each list consists of the indexes of the vertices
            /// of the corresponding connected component.
            /// The lists are arranged in ascending order with respect to
            /// the smallest index contained in the list.
            /// The indexes contained in each list are arranged in ascending order.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(5);
            /// uf.merge(1, 2);
            /// uf.merge(2, 3);
            /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3], vec![4]]);
            /// ```
            pub fn groups(&mut self) -> Vec<Vec<usize>> {
                let mut leader_to_idx: HashMap<usize, usize> = HashMap::new();
                let mut groups: Vec<Vec<usize>> = vec![];

                for i in 0..self.parent_or_size.len() {
                    let leader = self.leader(i);

                    if let Some(&idx) = leader_to_idx.get(&leader) {
                        groups[idx].push(i);
                    } else {
                        leader_to_idx.insert(leader, groups.len());
                        groups.push(vec![i]);
                    }
                }

                groups
            }

            /// Returns the size of the connected component that contains the vertex `a`.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.size(0), 1);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.size(0), 2);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.size(0), 3);
            /// ```
            pub fn size(&mut self, a: usize) -> usize {
                let leader = self.leader(a);
                -self.parent_or_size[leader] as usize
            }

            /// Adds a new vertex.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(4);
            /// uf.merge(1, 2);
            /// uf.merge(2, 3);
            /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3]]);
            /// uf.add();
            /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3], vec![4]]);
            /// ```
            pub fn add(&mut self) {
                self.parent_or_size.push(-1);
                self.group_num += 1;
            }

            /// Returns the number of connected components.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.group_num(), 3);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.group_num(), 2);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.group_num(), 1);
            /// ```
            pub fn group_num(&self) -> usize {
                self.group_num
            }

            /// Returns the number of elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(5);
            /// assert_eq!(uf.elem_num(), 5);
            /// ```
            pub fn elem_num(&self) -> usize {
                self.parent_or_size.len()
            }
        }
    }
}

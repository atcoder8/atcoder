use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

use crate::atcoder8_library::union_find::UnionFind;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let create_edge = |(i, j): (usize, usize)| {
        let x = aa[i];
        let y = aa[j];

        ((pow_mod(x, y, m) + pow_mod(y, x, m)) % m, i, j)
    };

    let edges = (0..n)
        .flat_map(|i| (0..n).map(move |j| create_edge((i, j))))
        .collect_vec();

    let mut ans = 0;
    let mut bh = BinaryHeap::from(edges);
    let mut uf = UnionFind::new(n);

    while let Some((weight, i, j)) = bh.pop() {
        if !uf.same(i, j) {
            ans += weight;
            uf.merge(i, j);
        }
    }

    println!("{}", ans);
}

pub fn pow_mod(x: usize, n: usize, m: usize) -> usize {
    assert!(m >= 1);

    let mut ret = 1 % m;
    let mut mul = x % m;
    let mut exp = n;

    while exp != 0 {
        if exp & 1 == 1 {
            ret = ret * mul % m;
        }

        mul = mul * mul % m;
        exp >>= 1;
    }

    ret
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

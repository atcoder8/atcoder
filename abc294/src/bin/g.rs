use itertools::{enumerate, rev, Itertools};
use proconio::{input, marker::Usize1};

use crate::fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1, i64); n - 1],
        q: usize,
    }

    #[derive(Debug, Clone, Copy)]
    struct Edge {
        idx: usize,
        from: usize,
        to: usize,
    }

    let mut graph = vec![vec![]; n];
    for (edge_idx, &(u, v, _weight)) in enumerate(&edges) {
        graph[u].push(Edge {
            idx: edge_idx,
            from: u,
            to: v,
        });

        graph[v].push(Edge {
            idx: edge_idx,
            from: v,
            to: u,
        });
    }

    enum DFSNode {
        Forward(Option<Edge>, usize),
        Backward(Edge, usize),
    }

    let mut in_per_node = vec![0; n];
    let mut out_per_node = vec![0; n];

    let mut in_per_edge = vec![0; n - 1];
    let mut out_per_edge = vec![0; n - 1];

    let mut parents = vec![None; n];

    let mut depths = vec![0; n];

    let mut stack = vec![DFSNode::Forward(None, 0)];
    let mut time = 0;
    while let Some(dfs_node) = stack.pop() {
        match dfs_node {
            DFSNode::Forward(edge, cur) => {
                in_per_node[cur] = time;

                if let Some(edge) = edge {
                    in_per_edge[edge.idx] = time;
                    parents[cur] = Some(edge.from);
                    depths[cur] = depths[edge.from] + 1;
                }

                for &next_edge in &graph[cur] {
                    if edge.is_some_and(|edge| next_edge.to == edge.from) {
                        continue;
                    }

                    stack.push(DFSNode::Backward(next_edge, next_edge.from));
                    stack.push(DFSNode::Forward(Some(next_edge), next_edge.to));
                }

                if edge.is_none() as usize + graph[cur].len() == 1 {
                    out_per_node[cur] = time;
                }
            }

            DFSNode::Backward(edge, cur) => {
                out_per_node[cur] = time;
                out_per_edge[edge.idx] = time;
            }
        }

        time += 1;
    }

    let mut doubling = vec![parents];
    for exp in (1_usize..).take_while(|exp| n >> exp != 0) {
        let ancestors = (0..n)
            .map(|cur| doubling[exp - 1][cur].and_then(|half| doubling[exp - 1][half]))
            .collect_vec();
        doubling.push(ancestors);
    }

    let find_lca = |mut node1: usize, mut node2: usize| {
        if depths[node1] > depths[node2] {
            std::mem::swap(&mut node1, &mut node2);
        }

        let diff = depths[node2] - depths[node1];
        for (exp, ancestors) in enumerate(&doubling) {
            if diff >> exp & 1 == 1 {
                node2 = ancestors[node2].unwrap();
            }
        }

        if node1 == node2 {
            return node1;
        }

        for ancestors in rev(&doubling) {
            if ancestors[node1] != ancestors[node2] {
                node1 = ancestors[node1].unwrap();
                node2 = ancestors[node2].unwrap();
            }
        }

        doubling[0][node1].unwrap()
    };

    let mut ft = FenwickTree::<i64>::new(time);
    for (edge_idx, &edge) in enumerate(&edges) {
        ft.add(in_per_edge[edge_idx], edge.2);
        ft.add(out_per_edge[edge_idx], -edge.2);
    }

    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (i, w): (Usize1, i64),
            }

            ft.set(in_per_edge[i], w);
            ft.set(out_per_edge[i], -w);
        } else {
            input! {
                (u, v): (Usize1, Usize1),
            }

            let ans = ft.sum(..=in_per_node[u]) + ft.sum(..=in_per_node[v])
                - 2 * ft.sum(..=in_per_node[find_lca(u, v)]);
            println!("{}", ans);
        }
    }
}

pub mod fenwick_tree {
    //! Processes the following query in `O(log(n))` time
    //! for a sequence of numbers with `n` elements:
    //! * Update one element
    //! * Calculate the sum of the elements of a range
    //! * Gets the elements of a number sequence.

    use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};

    /// # Examples
    ///
    /// ```
    /// use atcoder8_library::fenwick_tree::FenwickTree;
    ///
    /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
    /// assert_eq!(ft.sum(2..), 11);
    /// ```
    #[derive(Debug, Clone)]
    pub struct FenwickTree<T>(Vec<T>);

    impl<T> From<Vec<T>> for FenwickTree<T>
    where
        T: Default + Clone + AddAssign<T>,
    {
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        /// ```
        fn from(t: Vec<T>) -> Self {
            let mut ft = FenwickTree::new(t.len());
            for (i, x) in t.into_iter().enumerate() {
                ft.add(i, x);
            }

            ft
        }
    }

    impl<T> FenwickTree<T> {
        /// Constructs a `FenwickTree<T>` with `n` elements.
        ///
        /// Each element is initialized with `T::default()`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::<i32>::new(5);
        /// assert_eq!(ft.sum(..), 0);
        /// ```
        pub fn new(n: usize) -> Self
        where
            T: Default + Clone,
        {
            FenwickTree(vec![T::default(); n])
        }

        /// Add `x` to the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        ///
        /// ft.add(3, 100);
        /// assert_eq!(ft.sum(2..6), 109);
        /// ```
        pub fn add(&mut self, p: usize, x: T)
        where
            T: Clone + AddAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] += x.clone();
                p += p & p.overflowing_neg().0;
            }
        }

        /// Subtract `x` from the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::<u32>::from(vec![3, 1, 4, 1, 5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 19);
        ///
        /// ft.sub(3, 1);
        /// assert_eq!(ft.sum(2..6), 18);
        /// ```
        pub fn sub(&mut self, p: usize, x: T)
        where
            T: Clone + SubAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] -= x.clone();
                p += p & p.overflowing_neg().0;
            }
        }

        /// Sets `x` to the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        ///
        /// ft.set(3, 100);
        /// assert_eq!(ft.sum(2..6), 108);
        /// ```
        pub fn set(&mut self, p: usize, x: T)
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T> + SubAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            self.sub(p, self.get(p));
            self.add(p, x);
        }

        /// Compute the sum of the range [0, r).
        fn inner_sum(&self, r: usize) -> T
        where
            T: Default + Clone + AddAssign<T>,
        {
            let mut s = T::default();
            let mut r = r;
            while r > 0 {
                s += self.0[r - 1].clone();
                r -= r & r.wrapping_neg();
            }

            s
        }

        /// Calculate the total of the range.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(..), 13);
        /// assert_eq!(ft.sum(2..), 11);
        /// assert_eq!(ft.sum(..6), 11);
        /// assert_eq!(ft.sum(2..6), 9);
        /// assert_eq!(ft.sum(6..2), 0);
        /// ```
        pub fn sum<R>(&self, rng: R) -> T
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T>,
            R: RangeBounds<usize>,
        {
            let n = self.0.len();

            let l = match rng.start_bound() {
                std::ops::Bound::Included(&start_bound) => start_bound,
                std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let r = match rng.end_bound() {
                std::ops::Bound::Included(&end_bound) => end_bound + 1,
                std::ops::Bound::Excluded(&end_bound) => end_bound,
                std::ops::Bound::Unbounded => n,
            };

            assert!(l <= n && r <= n);

            if l >= r {
                T::default()
            } else {
                self.inner_sum(r) - self.inner_sum(l)
            }
        }

        /// Returns the value of an element in a sequence of numbers.
        /// Calculate the total of the range.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, -1, 5]);
        /// assert_eq!(ft.get(2), 4);
        /// ```
        pub fn get(&self, p: usize) -> T
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T>,
        {
            assert!(p < self.0.len());

            self.sum(p..=p)
        }
    }
}

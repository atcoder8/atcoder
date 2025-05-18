use fenwick_tree::FenwickTree;
use itertools::{enumerate, Itertools};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let edges = (0..n - 1).map(|i| Edge::read(i)).collect_vec();

    input! {
        q: usize,
    }

    let queries = (0..q).map(|_| Query::read()).collect_vec();

    let mut graph = vec![vec![]; n];
    for &edge in &edges {
        graph[edge.from].push(edge);
        graph[edge.to].push(edge.inverse());
    }

    let mut sorted_vertices = vec![];
    let mut range_by_edge = vec![n..n; n - 1];

    let mut stack = vec![(None::<Edge>, true)];
    let mut visited = vec![false; n];
    while let Some((edge, advance)) = stack.pop() {
        if advance {
            let curr_vertex = edge.map_or(0, |edge| edge.to);

            if visited[curr_vertex] {
                continue;
            }

            visited[curr_vertex] = true;

            if let Some(edge) = edge {
                range_by_edge[edge.id].start = sorted_vertices.len();
            }

            sorted_vertices.push(curr_vertex);

            stack.push((edge, false));
            stack.extend(
                graph[curr_vertex]
                    .iter()
                    .map(|&next_edge| (Some(next_edge), true)),
            );
        } else {
            if let Some(edge) = edge {
                range_by_edge[edge.id].end = sorted_vertices.len();
            }
        }
    }

    let mut labels = vec![0_usize; n];
    for (label, &vertex) in enumerate(&sorted_vertices) {
        labels[vertex] = label;
    }

    let mut total_weight = n;
    let mut ft = FenwickTree::from(vec![1; n]);
    for query in queries {
        match query {
            Query::Increase { vertex, add_weight } => {
                ft.add(labels[vertex], add_weight);
                total_weight += add_weight
            }
            Query::Question { edge } => {
                let sum_weight = ft.sum(range_by_edge[edges[edge].id].clone());
                println!("{}", total_weight.abs_diff(2 * sum_weight));
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    id: usize,
    from: usize,
    to: usize,
}

impl Edge {
    fn read(id: usize) -> Self {
        input! {
            (from, to): (Usize1, Usize1),
        }

        Self { id, from, to }
    }

    fn inverse(&self) -> Self {
        Self {
            id: self.id,
            from: self.to,
            to: self.from,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Increase { vertex: usize, add_weight: usize },
    Question { edge: usize },
}

impl Query {
    fn read() -> Self {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (vertex, add_weight): (Usize1, usize),
            }

            Self::Increase { vertex, add_weight }
        } else {
            input! {
                edge: Usize1,
            }

            Self::Question { edge }
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

use disjoint_intervals::DisjointIntervals;
use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

struct Edge {
    from: usize,
    to_left: usize,
    to_right: usize,
    cost: usize,
}

fn solve() -> Option<usize> {
    input! {
        (n, q): (usize, usize),
        lrc: [(Usize1, usize, usize); q],
    }

    let mut set = DisjointIntervals::new();

    let edges = enumerate(&lrc)
        .map(|(i, &(l, r, c))| Edge {
            from: n + i,
            to_left: l,
            to_right: r,
            cost: c,
        })
        .sorted_unstable_by_key(|edge| edge.cost);

    let mut uf = UnionFind::new(n + q);
    let mut sum_cost = 0;

    for edge in edges {
        if uf.merge(edge.from, edge.to_left) {
            sum_cost += edge.cost;
        }

        for i in set.range_exclusively(edge.to_left..edge.to_right - 1) {
            if uf.merge(i, i + 1) {
                sum_cost += edge.cost;
            }
        }

        set.insert_range(edge.to_left..edge.to_right - 1);
    }

    if uf.group_num() == 1 {
        Some(sum_cost)
    } else {
        None
    }
}

pub mod union_find {
    //! Union-Find processes the following queries on undirected graphs.
    //! * Merge two connected components.
    //! * Determine whether two given nodes are in the same connected component.
    //!
    //! To seed up processing, merge optimization using the number of nodes
    //! of the connected components and path compression are performed.
    //!
    //! The time complexity of each query is `O(A(n))`.
    //! where `n` is the number of nodes in the graph and
    //! `A(n)` is the inverse of the Ackermann function.

    /// This is the value that will be associated with each nodes of the graph.
    #[derive(Debug, Clone, Copy)]
    enum ParentOrSize {
        /// It is used for non-representative nodes and stores the parent node.
        Parent(usize),

        /// It is used for the representative node and
        /// stores the number of nodes of the connected component.
        Size(usize),
    }

    /// Union-Find processes the following queries on undirected graphs.
    /// * Merge two connected components.
    /// * Determine whether two given nodes are in the same connected component.
    ///
    /// To seed up processing, merge optimization using the number of nodes
    /// of the connected components and path compression are performed.
    ///
    /// The time complexity of each query is `O(A(n))`.
    /// where `n` is the number of nodes in the graph and
    /// `A(n)` is the inverse of the Ackermann function.
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
    #[derive(Debug, Default, Clone)]
    pub struct UnionFind {
        /// For each node, one of the following is stored.
        /// * The number of nodes of the connected component to which it belongs.
        /// (If it is a representative node of the connected component.)
        /// * Index of the parent node. (Otherwise.)
        parent_or_size: Vec<ParentOrSize>,

        /// Number of connected components.
        group_num: usize,
    }

    impl UnionFind {
        /// Create an undirected graph with `n` nodes and `0` edges.
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
                parent_or_size: vec![ParentOrSize::Size(1); n],
                group_num: n,
            }
        }

        /// Return the representative node of the connected component containing node `a`.
        ///
        /// At that time, perform path compression on the nodes on the path from node `a` to the representative node.
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
            // If node `a` is a representative node of the connected component, return `a`.
            if let ParentOrSize::Size(_) = self.parent_or_size[a] {
                return a;
            }

            // Path from node `a` to the representative node.
            let mut path = vec![];

            // Current node.
            let mut current = a;

            // Record the path to the representative node.
            while let ParentOrSize::Parent(parent) = self.parent_or_size[current] {
                // Add current node to the path.
                path.push(current);

                // Move to the parent node.
                current = parent;
            }

            // The representative node of the connected component.
            let leader = current;

            // Set nodes on the path as direct children of the representative node.
            path.iter().for_each(|&node| {
                self.parent_or_size[node] = ParentOrSize::Parent(leader);
            });

            // Return the representative node of the connected component.
            leader
        }

        /// Return whether two nodes `a` and `b` are in the same connected component.
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

        /// Merge each connected component containing nodes `a` and `b`.
        ///
        /// Return `true` if different connected components are newly merged.
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
            // Representative node of the connected component that contains the node `a`.
            let leader_a = self.leader(a);
            // Representative node of the connected component that contains the node `b`.
            let leader_b = self.leader(b);

            // If nodes `a` and `b` are in the same connected component, return `false` without processing.
            if leader_a == leader_b {
                return false;
            }

            // Number of nodes of the component containing node `a`.
            let component_size_a = self.size(leader_a);

            // Number of nodes of the component containing node `b`.
            let component_size_b = self.size(leader_b);

            // Number of nodes of the merged component.
            let merged_component_size = component_size_a + component_size_b;

            // Set the parent of the representative node of the smaller sized connected component
            // to be the parent of the other connected component.
            if component_size_a <= component_size_b {
                self.parent_or_size[leader_a] = ParentOrSize::Parent(leader_b);
                self.parent_or_size[leader_b] = ParentOrSize::Size(merged_component_size);
            } else {
                self.parent_or_size[leader_b] = ParentOrSize::Parent(leader_a);
                self.parent_or_size[leader_a] = ParentOrSize::Size(merged_component_size);
            }

            // Decrease the number of connected components by one.
            self.group_num -= 1;

            // Return `true` because different connected components are newly combined.
            true
        }

        /// Return a list of connected components.
        ///
        /// Each connected component consists of indexes of nodes.
        /// The indexes of the nodes in each connected component are arranged in ascending order.
        /// The list of connected components is sorted in ascending order
        /// with respect to the smallest index of the included nodes.
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
            // Number of nodes in graph.
            let element_num = self.parent_or_size.len();

            // List of connected components.
            let mut groups: Vec<Vec<usize>> = vec![];
            // Correspondence between the representative node and group index.
            let mut leader_to_idx: Vec<Option<usize>> = vec![None; element_num];

            // Assign each node in the graph to a group.
            for node in 0..element_num {
                // Representative node of the connected component to which the `node` belongs.
                let leader = self.leader(node);

                if let Some(group_idx) = leader_to_idx[leader] {
                    // Assign to an existing group.
                    groups[group_idx].push(node);
                } else {
                    // Adding a new group.
                    leader_to_idx[leader] = Some(groups.len());
                    groups.push(vec![node]);
                }
            }

            // Return a list of groups.
            groups
        }

        /// Return the number of nodes in the connected component to which node `a` belongs.
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

            match self.parent_or_size[leader] {
                ParentOrSize::Parent(_) => panic!(),
                ParentOrSize::Size(size) => size,
            }
        }

        /// Add a new node with degree `0`.
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
            self.parent_or_size.push(ParentOrSize::Size(1));
            self.group_num += 1;
        }

        /// Return the number of connected components.
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

        /// Return the number of nodes in the graph.
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

pub mod disjoint_intervals {
    use std::{collections::BTreeMap, ops};

    /// Data structure that represent a set by the sum of disjoint intervals.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DisjointIntervals<T> {
        /// Map with the start of the interval as key and the end of the interval as value.
        /// Each interval represented as a right half-open interval.
        intervals: BTreeMap<T, T>,

        /// Number of elements belonging to one of the intervals.
        len: usize,
    }

    impl DisjointIntervals<usize> {
        /// Creates a new empty set.
        pub fn new() -> Self {
            DisjointIntervals {
                intervals: BTreeMap::new(),
                len: 0,
            }
        }

        /// Returns the number of elements in the set.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if and only if the set is empty.
        pub fn is_empty(&self) -> bool {
            self.intervals.is_empty()
        }

        /// Returns `true` if and only if `value` is in the set.
        pub fn contains(&self, value: usize) -> bool {
            self.intervals
                .range(..=value)
                .next_back()
                .is_some_and(|(_, &right)| right > value)
        }

        /// Returns the number of non-contiguous intervals.
        pub fn number_of_intervals(&self) -> usize {
            self.intervals.len()
        }

        /// Adds the elements contained in the `range`.
        /// Returns the number of newly added elements.
        pub fn insert_range(&mut self, range: ops::Range<usize>) -> usize {
            if range.is_empty() {
                return 0;
            }

            // Finds both ends of the updated intervals such that they contain `range`.
            let insert_left = match self.intervals.range(..=range.start).next_back() {
                Some((&left, &right)) if right >= range.start => left,
                _ => range.start,
            };
            let insert_right = match self.intervals.range(..=range.end).next_back() {
                Some((_, &right)) => right.max(range.end),
                None => range.end,
            };

            // Update the set of intervals and counts the number of newly inserted elements.
            let mut add_element_num = insert_right - insert_left;
            loop {
                match self.intervals.range(insert_left..).next() {
                    Some((&left, &right)) if right <= insert_right => {
                        self.intervals.remove(&left);
                        add_element_num -= right - left;
                    }
                    _ => break,
                }
            }
            self.intervals.insert(insert_left, insert_right);

            // Update number of elements in the set.
            self.len += add_element_num;

            add_element_num
        }

        /// Adds an element.
        /// Returns `true` if and only if the element is newly added.
        pub fn insert(&mut self, value: usize) -> bool {
            self.insert_range(value..value + 1) == 1
        }

        /// Returns the smallest non-negative integer not included in the set.
        pub fn mex(&self) -> usize {
            match self.intervals.first_key_value() {
                Some((&left, &right)) if left == 0 => right,
                _ => 0,
            }
        }

        /// Generates an iterator that traverses the elements contained in the set.
        pub fn range_inclusively(&self, range: ops::Range<usize>) -> RangeInclusively<usize> {
            RangeInclusively {
                intervals: self,
                range,
            }
        }

        /// Generates an iterator that traverses the elements not contained in the set.
        pub fn range_exclusively(&self, range: ops::Range<usize>) -> RangeExclusively<usize> {
            RangeExclusively {
                intervals: self,
                range,
            }
        }
    }

    #[derive(Debug)]
    pub struct RangeInclusively<'a, T> {
        intervals: &'a DisjointIntervals<T>,
        range: ops::Range<usize>,
    }

    impl<'a> Iterator for RangeInclusively<'a, usize> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if !self.intervals.contains(self.range.start) {
                match self.intervals.intervals.range(self.range.start..).next() {
                    Some((&left, _)) => self.range.start = left,
                    None => {
                        self.range.start = self.range.end;
                        return None;
                    }
                }
            }

            self.range.start += 1;

            Some(self.range.start - 1)
        }
    }

    impl<'a> DoubleEndedIterator for RangeInclusively<'a, usize> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if !self.intervals.contains(self.range.end - 1) {
                match self.intervals.intervals.range(..self.range.end).next_back() {
                    Some((_, &right)) => self.range.end = right,
                    None => {
                        self.range.end = self.range.start;
                        return None;
                    }
                }
            }

            self.range.end -= 1;

            Some(self.range.end)
        }
    }

    #[derive(Debug)]
    pub struct RangeExclusively<'a, T> {
        intervals: &'a DisjointIntervals<T>,
        range: ops::Range<usize>,
    }

    impl<'a> Iterator for RangeExclusively<'a, usize> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if self.intervals.contains(self.range.start) {
                let (_, &right) = self
                    .intervals
                    .intervals
                    .range(..=self.range.start)
                    .next_back()
                    .unwrap();
                self.range.start = right.min(self.range.end);

                if self.range.is_empty() {
                    return None;
                }
            }

            self.range.start += 1;

            Some(self.range.start - 1)
        }
    }

    impl<'a> DoubleEndedIterator for RangeExclusively<'a, usize> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if self.intervals.contains(self.range.end - 1) {
                let (&left, _) = self
                    .intervals
                    .intervals
                    .range(..self.range.end)
                    .next_back()
                    .unwrap();
                self.range.end = left.max(self.range.start);

                if self.range.is_empty() {
                    return None;
                }
            }

            self.range.end -= 1;

            Some(self.range.end)
        }
    }
}

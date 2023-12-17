// unfinished

use std::collections::{BTreeSet, BinaryHeap};

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    println!("{}", solve(k, aa));
}

fn solve(k: usize, mut aa: Vec<usize>) -> usize {
    let n = aa.len();

    aa.sort_unstable();

    let calc_rem = |v1: usize, v2: usize| (v2 + k - v1) % k;

    let mut sections = BTreeSet::new();
    for (i, &a) in enumerate(&aa) {
        sections.insert((a, a, i));
    }

    let mut heap = BinaryHeap::new();
    for &(_, right1, id1) in &sections {
        let (left2, right2, id2) = *sections
            .range(..(right1 - 1, k, 0))
            .rev()
            .find(|&&section| section.2 != id1)
            .or(sections
                .range((right1, 0, 0)..)
                .rev()
                .find(|&&section| section.2 != id1))
            .unwrap();

        heap.push((calc_rem(right1, left2), id1, id2, right2));
    }

    let mut ans = 0;
    let mut flags = vec![true; n];
    while let Some((rem, id1, id2, right1)) = heap.pop() {
        if !flags[id1] || !flags[id2] {
            continue;
        }

        flags[id1] = false;
        flags[id2] = false;

        ans += rem;

        let (left2, right2, id2) = *sections
            .range(..(right1 - 1, k, 0))
            .rev()
            .find(|&&section| section.2 != id1)
            .or(sections
                .range((right1, 0, 0)..)
                .rev()
                .find(|&&section| section.2 != id1))
            .unwrap();

        heap.push((calc_rem(right1, left2), flags.len(), id2, right2));
        flags.push(true);
    }

    ans
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::Rng;

    /// Input data type.
    type Input = (usize, Vec<usize>);

    /// Output data type.
    type Output = usize;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let expected_output = expected(input.clone());
            let actual_output = solve(input.clone());

            assert_eq!(
                expected_output, actual_output,
                "
Wrong Answer on Test #{}

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
                test_case_index, input, expected_output, actual_output
            );
        }
    }

    /// Generate test cases.
    pub fn generator() -> Input {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(2..=4);
        let k = rng.gen_range(1..=5);
        let aa = (0..n).map(|_| rng.gen_range(0..k)).collect();

        (k, aa)
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let (k, aa) = input;
        let n = aa.len();

        let calc_score = |perm: &[usize]| {
            perm.windows(2)
                .map(|window| (aa[window[1]] + k - aa[window[0]]) % k)
                .sum::<usize>()
        };

        let best_perm = (0..n)
            .permutations(n)
            .max_by_key(|perm| calc_score(perm))
            .unwrap();

        eprintln!(
            "sorted_aa = [{}], ans = {}",
            best_perm.iter().map(|&idx| aa[idx]).join(", "),
            calc_score(&best_perm)
        );

        calc_score(&best_perm)
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let (k, aa) = input;

        crate::solve(k, aa)
    }

    #[test]
    fn test2() {
        assert_eq!(expected((5, vec![2, 1, 4, 1])), 10);
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

use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

use crate::union_find::UnionFind;

fn main() {
    input! {
        (n, q): (usize, usize),
        abd: [(Usize1, Usize1, i64); q],
    }

    let mut ans = vec![];
    let mut uf = UnionFind::new(n);

    for (i, &(a, b, d)) in enumerate(&abd) {
        if uf.merge(a, b, d).is_ok() {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.iter().join(" "));
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
    #[derive(Debug, Default, Clone)]
    pub struct UnionFind {
        /// For each node, one of the following is stored.
        /// * The number of nodes of the connected component to which it belongs.
        /// (If it is a representative node of the connected component.)
        /// * Index of the parent node. (Otherwise.)
        parent_or_size: Vec<ParentOrSize>,

        /// Number of connected components.
        group_num: usize,

        potentials: Vec<i64>,
    }

    impl UnionFind {
        /// Create an undirected graph with `n` nodes and `0` edges.
        pub fn new(n: usize) -> Self {
            UnionFind {
                parent_or_size: vec![ParentOrSize::Size(1); n],
                group_num: n,
                potentials: vec![0; n],
            }
        }

        /// Return the representative node of the connected component containing node `a`.
        ///
        /// At that time, perform path compression on the nodes on the path from node `a` to the representative node.
        pub fn leader(&mut self, a: usize) -> usize {
            // If node `a` is a representative node of the connected component, return `a`.
            if let ParentOrSize::Size(_) = self.parent_or_size[a] {
                return a;
            }

            // Path from node `a` to the representative node.
            let mut path = vec![a];

            // Current node.
            let mut current = a;

            // Record the path to the representative node.
            while let ParentOrSize::Parent(parent) = self.parent_or_size[current] {
                // Add current node to the path.
                path.push(parent);

                // Move to the parent node.
                current = parent;
            }

            path.reverse();

            // The representative node of the connected component.
            let leader = current;

            // Set nodes on the path as direct children of the representative node.
            for pair in path.windows(2) {
                self.parent_or_size[pair[1]] = ParentOrSize::Parent(leader);
                self.potentials[pair[1]] += self.potentials[pair[0]];
            }

            // Return the representative node of the connected component.
            leader
        }

        /// Return whether two nodes `a` and `b` are in the same connected component.
        ///
        /// # Examples
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.leader(a) == self.leader(b)
        }

        pub fn potential(&mut self, a: usize, b: usize) -> Option<i64> {
            let leader_a = self.leader(a);
            let leader_b = self.leader(b);

            if leader_a == leader_b {
                Some(self.potentials[b] - self.potentials[a])
            } else {
                None
            }
        }

        /// Merge each connected component containing nodes `a` and `b`.
        ///
        /// Return `true` if different connected components are newly merged.
        pub fn merge(&mut self, a: usize, b: usize, potential_diff: i64) -> Result<bool, i64> {
            // Representative node of the connected component that contains the node `a`.
            let leader_a = self.leader(a);
            // Representative node of the connected component that contains the node `b`.
            let leader_b = self.leader(b);

            // If nodes `a` and `b` are in the same connected component, return `false` without processing.
            if leader_a == leader_b {
                if self.potentials[b] - self.potentials[a] == potential_diff {
                    return Ok(false);
                } else {
                    return Err(self.potentials[b] - self.potentials[a]);
                }
            }

            // Number of nodes of the component containing node `a`.
            let component_size_a = self.size(leader_a);

            // Number of nodes of the component containing node `b`.
            let component_size_b = self.size(leader_b);

            // Number of nodes of the merged component.
            let merged_component_size = component_size_a + component_size_b;

            // Set the parent of the representative node of the smaller sized connected component
            // to be the parent of the other connected component.
            let leader_potential_diff = potential_diff + self.potentials[a] - self.potentials[b];
            if component_size_a <= component_size_b {
                self.parent_or_size[leader_a] = ParentOrSize::Parent(leader_b);
                self.parent_or_size[leader_b] = ParentOrSize::Size(merged_component_size);
                self.potentials[leader_a] = -leader_potential_diff;
            } else {
                self.parent_or_size[leader_b] = ParentOrSize::Parent(leader_a);
                self.parent_or_size[leader_a] = ParentOrSize::Size(merged_component_size);
                self.potentials[leader_b] = leader_potential_diff;
            }

            // Decrease the number of connected components by one.
            self.group_num -= 1;

            // Return `true` because different connected components are newly combined.
            Ok(true)
        }

        /// Return a list of connected components.
        ///
        /// Each connected component consists of indexes of nodes.
        /// The indexes of the nodes in each connected component are arranged in ascending order.
        /// The list of connected components is sorted in ascending order
        /// with respect to the smallest index of the included nodes.
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
        pub fn size(&mut self, a: usize) -> usize {
            let leader = self.leader(a);

            match self.parent_or_size[leader] {
                ParentOrSize::Parent(_) => panic!(),
                ParentOrSize::Size(size) => size,
            }
        }

        /// Add a new node with degree `0`.
        pub fn add(&mut self) {
            self.parent_or_size.push(ParentOrSize::Size(1));
            self.group_num += 1;
        }

        /// Return the number of connected components.
        pub fn group_num(&self) -> usize {
            self.group_num
        }

        /// Return the number of nodes in the graph.
        pub fn elem_num(&self) -> usize {
            self.parent_or_size.len()
        }
    }
}

use crate::atcoder8_library::union_find::UnionFind;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut uv = Vec::new();
    for _ in 0..n {
        uv.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        });
    }
    let mut graph = vec![vec![]; n as usize];
    for &(u, v) in uv.iter() {
        graph[u as usize].push(v);
        graph[v as usize].push(u);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());
    let q = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut xy = Vec::new();
    for _ in 0..q {
        xy.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        });
    }

    let mut visited = vec![false; n];

    let (cycle, _cycle_start, _cycle_flag) = dfs(&graph, &mut visited, None, 0).unwrap();
    let mut included_closed = vec![false; n];
    for &elem in cycle.iter() {
        included_closed[elem] = true;
    }

    let mut uf = UnionFind::new(n);
    for &(u, v) in uv.iter() {
        if !included_closed[u] || !included_closed[v] {
            uf.merge(u, v);
        }
    }

    for &(x, y) in xy.iter() {
        println!("{}", if uf.same(x, y) { "Yes" } else { "No" });
    }
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    parent: Option<usize>,
    curr: usize,
) -> Option<(Vec<usize>, usize, bool)> {
    if visited[curr] {
        return Some((vec![curr], curr, true));
    }

    visited[curr] = true;

    for &next in graph[curr].iter() {
        if Some(next) == parent {
            continue;
        }

        let res = dfs(graph, visited, Some(curr), next);
        if let Some((mut cycle, cycle_start, cycle_flag)) = res {
            if cycle_flag {
                cycle.push(curr);
            }
            let next_cycle_flag = cycle_flag && curr != cycle_start;

            return Some((cycle, cycle_start, next_cycle_flag));
        }
    }

    None
}

pub mod atcoder8_library {
    pub mod union_find {
        //! Union-Find processes the following queries for an edgeless graph in `O(α(n))` amortized time.
        //! * Add an undirected edge.
        //! * Deciding whether given two vertices are in the same connected component
        //!
        //! When a method is called, route compression is performed as appropriate.

        /// Union-Find processes the following queries for an edgeless graph in `O(α(n))` amortized time.
        /// * Add an undirected edge.
        /// * Deciding whether given two vertices are in the same connected component
        ///
        /// When a method is called, route compression is performed as appropriate.
        ///
        /// # Examples
        ///
        /// ```
        /// use union_find::UnionFind;
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

            /// If it is not representative, the index of its parent is stored.
            group_num: usize,
        }

        impl UnionFind {
            /// It creates an undirected graph with `n` vertices and 0 edges.
            pub fn new(n: usize) -> Self {
                UnionFind {
                    parent_or_size: vec![-1; n],
                    group_num: n,
                }
            }

            /// It returns the representative of the connected component that contains the vertex `a`.
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

                // Returns a representative of the connected component.
                curr
            }

            /// It returns whether the vertices `a` and `b` are in the same connected component.
            pub fn same(&mut self, a: usize, b: usize) -> bool {
                self.leader(a) == self.leader(b)
            }

            /// It adds an edge between vertex `a` and vertex `b`.
            /// It returns true if the connected component to which a belongs and that of b are newly combined.
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

                // Returns true because the connected component is newly combined.
                true
            }

            /// It returns the size of the connected component that contains the vertex `a`.
            pub fn size(&mut self, a: usize) -> usize {
                let leader = self.leader(a);
                -self.parent_or_size[leader] as usize
            }

            /// It adds a new vertex.
            pub fn add(&mut self) {
                self.parent_or_size.push(-1);
                self.group_num += 1;
            }

            /// It returns the number of connected components.
            pub fn get_group_num(&self) -> usize {
                self.group_num
            }

            /// It returns the number of elements.
            pub fn get_elem_num(&self) -> usize {
                self.parent_or_size.len()
            }
        }
    }
}

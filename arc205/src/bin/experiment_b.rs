use std::collections::BTreeSet;

fn main() {
    const NUM_VERTICES: usize = 5;
    const NUM_EDGES: usize = NUM_VERTICES * (NUM_VERTICES - 1) / 2;

    for edge_colors in 0..1_u64 << NUM_EDGES {
        let init_graph = ColoredGraph {
            num_vertices: NUM_VERTICES,
            edge_colors,
        };
        let max_black_edges = find_max_black_edges(init_graph);
        println!("{:?}: {max_black_edges}", init_graph.black_edges());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ColoredGraph {
    num_vertices: usize,
    edge_colors: u64,
}

impl ColoredGraph {
    // fn new(num_vertices: usize) -> Self {
    //     Self {
    //         num_vertices,
    //         edge_colors: 0,
    //     }
    // }

    fn edge_index(&self, mut u: usize, mut v: usize) -> usize {
        assert!(u < self.num_vertices && v < self.num_vertices && u != v);
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        (0..u).map(|i| self.num_vertices - 1 - i).sum::<usize>() + v - 1 - u
    }

    fn flip_edge_color(&mut self, u: usize, v: usize) {
        let edge_index = self.edge_index(u, v);
        self.edge_colors ^= 1 << edge_index;
    }

    fn num_black_edges(&self) -> u32 {
        self.edge_colors.count_ones()
    }

    fn is_black_edge(&self, u: usize, v: usize) -> bool {
        let edge_index = self.edge_index(u, v);
        self.edge_colors >> edge_index & 1 == 1
    }

    fn black_edges(&self) -> Vec<(usize, usize)> {
        let mut black_edges = vec![];
        for u in 0..self.num_vertices {
            for v in u + 1..self.num_vertices {
                if self.is_black_edge(u, v) {
                    black_edges.push((u, v));
                }
            }
        }

        black_edges
    }
}

fn find_max_black_edges(init_graph: ColoredGraph) -> u32 {
    let mut memo: BTreeSet<ColoredGraph> = BTreeSet::new();
    let mut stack = vec![init_graph];
    while let Some(graph) = stack.pop() {
        if memo.contains(&graph) {
            continue;
        }

        memo.insert(graph);
        for i in 0..init_graph.num_vertices {
            for j in i + 1..init_graph.num_vertices {
                for k in j + 1..init_graph.num_vertices {
                    let mut next_graph = graph;
                    next_graph.flip_edge_color(i, j);
                    next_graph.flip_edge_color(i, k);
                    next_graph.flip_edge_color(j, k);
                    stack.push(next_graph);
                }
            }
        }
    }

    memo.iter()
        .map(|graph| graph.num_black_edges())
        .max()
        .unwrap()
}

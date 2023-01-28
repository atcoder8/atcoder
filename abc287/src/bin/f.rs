// unfinished

use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut degrees = vec![0; n];

    for &(a, b) in &ab {
        degrees[a] += 1;
        degrees[b] += 1;
    }

    let leaf_nodes = degrees.iter().enumerate().filter(|(_, &degree)| degree == 1).map(|(i, _)| i).collect_vec();

    // let start_node = leaf_nodes[0];
    // let queue = VecDeque::from(vec![start_node]);

    // while let Some(cur_node) = queue.pop_front() {
        
    // }

    // let dp = vec![0; n];

    // for i in 0..n {
    //     let mut next_dp = vec![0; n];

    //     for  in  {
            
    //     }
    // }
}

fn dfs(graph: &Vec<Vec<usize>>, prev_node: Option<usize>, cur_node: usize, degrees: &Vec<usize>, dp: &Vec<usize>) {
    if degrees[cur_node] == 1 {
        return;
    }

    let n = graph.len();
    
    let next_dp = vec![0; n];

    for &next_node in &graph[cur_node] {
        if Some(next_node) != prev_node {
            
        }
    }
}

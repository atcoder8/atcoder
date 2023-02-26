use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    if let Some(ans) = solve() {
        println!("Yes\n{}", join(ans, " "));
    } else {
        println!("No");
    }
}

fn solve() -> Option<Vec<usize>> {
    input! {
        (n, m): (usize, usize),
        xy: [(Usize1, Usize1); m],
    }

    let mut indegree = vec![0; n];
    let mut graph = vec![vec![]; n];
    for &(a, b) in &xy {
        indegree[b] += 1;
        graph[a].push(b);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let tail_node_vec = (0..n).filter(|&i| indegree[i] == 0).collect_vec();

    if tail_node_vec.len() != 1 {
        return None;
    }

    let tail_node = tail_node_vec[0];

    let mut order = vec![tail_node];

    let mut cur_node = tail_node;

    for _ in 1..n {
        let mut par_node = None;

        for &ancestor_node in &graph[cur_node] {
            indegree[ancestor_node] -= 1;

            if indegree[ancestor_node] == 0 {
                if par_node.is_some() {
                    return None;
                }

                par_node = Some(ancestor_node);
            }
        }

        cur_node = par_node.unwrap();

        order.push(cur_node);
    }

    let mut ans = vec![0; n];
    for (i, &ord) in order.iter().enumerate() {
        ans[ord] = i + 1;
    }

    Some(ans)
}

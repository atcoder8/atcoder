use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n - 1],
        m: usize,
        mut aa: [Usize1; m],
    }

    let mut graph = vec![vec![]; n];
    for (i, &p) in pp.iter().enumerate() {
        graph[p].push(i + 1);
    }

    let mut dp = vec![0; n];
    for &a in &aa {
        dp[a] += 1;
    }

    let ans = calc_cost(&graph, &mut dp, 0);

    println!("{}", ans);
}

fn calc_cost(graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>, node: usize) -> usize {
    let mut cost = 0;

    for &next_node in &graph[node] {
        cost += calc_cost(graph, dp, next_node);
        let move_node_num = dp[next_node].saturating_sub(1);
        cost += move_node_num;
        dp[node] += move_node_num;
    }

    cost
}

use std::io::Write;

fn main() {
    let (n, _m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut adj_list = vec![vec![false; n]; n];
    let mut visited = vec![false; n];
    visited[0] = true;

    dfs(&mut adj_list, &mut visited, 0);
}

fn read_adjacent_nodes() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut split_line = line.split_whitespace();

    let msg = split_line.next().unwrap();
    if msg == "OK" {
        std::process::exit(0);
    }
    if msg == "-1" {
        std::process::exit(1);
    }

    split_line
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect()
}

fn dfs(adj_list: &mut Vec<Vec<bool>>, visited: &mut Vec<bool>, cur: usize) {
    let adj_nodes = read_adjacent_nodes();

    for &node in &adj_nodes {
        adj_list[cur][node] = true;
    }

    for next in 0..adj_list.len() {
        if !adj_list[cur][next] || visited[next] {
            continue;
        }

        visited[next] = true;

        println!("{}", next + 1);
        std::io::stdout().flush().unwrap();

        dfs(adj_list, visited, next);

        println!("{}", cur + 1);

        read_adjacent_nodes();
        std::io::stdout().flush().unwrap();
    }
}

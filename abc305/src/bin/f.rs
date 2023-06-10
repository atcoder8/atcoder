// unfinished

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

    let mut adj_mat = vec![vec![false; n]; n];
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut stack: Vec<(usize, bool)> = vec![(0, false)];
    let mut first_time = true;

    while let Some((cur, back)) = stack.pop() {
        if !first_time {
            println!("{}", cur + 1);
            std::io::stdout().flush().unwrap();
        } else {
            first_time = false;
        }

        let adj_nodes: Vec<_> = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut split_line = line.split_whitespace();

            let msg = split_line.next().unwrap();
            if msg == "OK" {
                break;
            }
            if msg == "-1" {
                std::process::exit(1);
            }

            split_line
                .map(|x| x.parse::<usize>().unwrap() - 1)
                .collect()
        };

        for &adj_node in &adj_nodes {
            adj_mat[cur][adj_node] = true;
            adj_mat[adj_node][cur] = true;
        }

        if back {
            continue;
        }

        stack.push((cur, true));

        for next in (0..n).filter(|&i| adj_mat[cur][i]) {
            if visited[next] {
                continue;
            }

            visited[next] = true;
            stack.push((next, false));
        }
    }
}

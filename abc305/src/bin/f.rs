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
    let mut stack = vec![(0, false)];
    while let Some((cur, back)) = stack.pop() {
        if (cur, back) != (0, false) {
            println!("{}", cur + 1);
            std::io::stdout().flush().unwrap();
        }

        let adj_nodes = read_adjacent_nodes();

        if back {
            continue;
        }

        if visited[cur] {
            stack.pop();

            continue;
        }

        visited[cur] = true;

        for &node in &adj_nodes {
            adj_list[cur][node] = true;
        }

        for next in 0..n {
            if !adj_list[cur][next] || visited[next] {
                continue;
            }

            stack.push((cur, true));
            stack.push((next, false));
        }
    }
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

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abxy: [(Usize1, Usize1, i64, i64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, x, y) in &abxy {
        graph[a].push((b, (x, y)));
        graph[b].push((a, (-x, -y)));
    }

    let mut coords: Vec<Option<(i64, i64)>> = vec![None; n];
    let mut stack = vec![(0, (0, 0))];
    while let Some((cur, cur_coord)) = stack.pop() {
        if coords[cur].is_some() {
            continue;
        }

        coords[cur] = Some(cur_coord);

        let (cur_x, cur_y) = cur_coord;
        for &(next, (diff_x, diff_y)) in &graph[cur] {
            stack.push((next, (cur_x + diff_x, cur_y + diff_y)));
        }
    }

    for &coord in &coords {
        match coord {
            Some((x, y)) => println!("{x} {y}"),
            None => println!("undecidable"),
        }
    }
}

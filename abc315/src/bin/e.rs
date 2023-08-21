use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut ppp: [[Usize1]; n],
    }

    let mut books = vec![];
    dfs(&ppp, 0, &mut vec![false; n], &mut books);
    println!("{}", join(books.iter().map(|&book| book + 1), " "));
}

fn dfs(graph: &[Vec<usize>], cur: usize, visited: &mut [bool], books: &mut Vec<usize>) {
    for &next in &graph[cur] {
        if visited[next] {
            continue;
        }

        visited[next] = true;
        dfs(graph, next, visited, books);
        books.push(next);
    }
}

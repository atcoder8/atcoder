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

    let mut cnt = 0;
    rec(&graph, None, 0, &mut cnt);
    println!("{}", cnt);
}

fn rec(graph: &Vec<Vec<usize>>, par: Option<usize>, cur: usize, cnt: &mut usize) -> usize {
    let mut descendants = vec![];

    for &next in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        descendants.push(rec(graph, Some(cur), next, cnt));
    }

    let child_num = descendants.len();

    for i in 0..child_num {
        for j in (i + 1)..child_num {
            for k in (j + 1)..child_num {
                *cnt += descendants[i] * descendants[j] * descendants[k];
            }
        }
    }

    descendants.iter().sum::<usize>() + 1
}

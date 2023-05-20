use im_rc::HashSet;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut ans = n;
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                (u, v): (Usize1, Usize1),
            }

            ans -= (graph[u].is_empty()) as usize + (graph[v].is_empty()) as usize;

            graph[u].insert(v);
            graph[v].insert(u);
        } else {
            input! {
                v: Usize1,
            }

            if !graph[v].is_empty() {
                let others = graph[v].iter().cloned().collect_vec();
                for u in others {
                    if graph[u].contains(&v) {
                        graph[u].remove(&v);
                    }

                    if graph[u].is_empty() {
                        ans += 1;
                    }
                }

                graph[v].clear();
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}

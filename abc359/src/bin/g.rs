use hashbag::HashBag;
use itertools::Itertools;
use once_cell::sync::OnceCell;

use proconio::{input, marker::Usize1};

#[derive(Debug)]
struct Global {
    graph: Vec<Vec<usize>>,
    labels: Vec<usize>,
}

impl Global {
    fn new() -> Self {
        input! {
            n: usize,
            uv: [(Usize1, Usize1); n - 1],
            aa: [Usize1; n],
        }

        let mut graph = vec![vec![]; n];
        for &(u, v) in &uv {
            graph[u].push(v);
            graph[v].push(u);
        }

        Self { graph, labels: aa }
    }
}

static INSTANCE: OnceCell<Global> = OnceCell::new();

fn main() {
    INSTANCE.set(Global::new()).unwrap();

    let n = INSTANCE.get().unwrap().graph.len();
    let mut cnt_each_label = vec![0; n];
    for &label in &INSTANCE.get().unwrap().labels {
        cnt_each_label[label] += 1;
    }

    let mut ans = 0;
    rec(None, 0, &cnt_each_label, &mut ans);
    println!("{}", ans);
}

fn rec(
    par: Option<usize>,
    cur: usize,
    cnt_each_label: &[usize],
    ans: &mut usize,
) -> HashBag<usize> {
    let bags = INSTANCE.get().unwrap().graph[cur]
        .iter()
        .filter_map(|&next| {
            if Some(next) != par {
                Some(rec(Some(cur), next, cnt_each_label, ans))
            } else {
                None
            }
        })
        .collect_vec();

    for bag in &bags {
        for (&label, cnt) in bag.set_iter() {
            *ans += cnt * (cnt_each_label[label] - cnt);
        }
    }

    let mut merged_bag = HashBag::new();
    let label = INSTANCE.get().unwrap().labels[cur];
    merged_bag.insert(label);
    for mut bag in bags {
        if merged_bag.len() < bag.len() {
            std::mem::swap(&mut merged_bag, &mut bag);
        }

        for (&label, cnt) in bag.set_iter() {
            merged_bag.insert_many(label, cnt);
        }
    }

    merged_bag
}

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut cycles = vec![];
    let mut labels: Vec<Option<usize>> = vec![None; n];

    for start in 0..n {
        if labels[start].is_some() {
            continue;
        }

        let mut cur = start;
        while labels[cur].is_none() {
            labels[cur] = Some(start);
            cur = aa[cur];
        }

        if labels[cur] != Some(start) {
            continue;
        }

        let mut cycle = vec![cur];
        let cycle_start = cur;
        cur = aa[cur];

        while cur != cycle_start {
            cycle.push(cur);
            cur = aa[cur];
        }

        cycles.push(cycle);
    }

    let mut counts_each_node = vec![None; n];
    for cycle in &cycles {
        for &node in cycle {
            counts_each_node[node] = Some(cycle.len());
        }
    }

    for start in 0..n {
        rec(&aa, start, &mut counts_each_node);
    }

    let ans = counts_each_node
        .iter()
        .map(|cnt| cnt.unwrap())
        .sum::<usize>();
    println!("{}", ans);
}

fn rec(aa: &[usize], cur: usize, counts_each_node: &mut [Option<usize>]) -> usize {
    if let Some(cnt) = counts_each_node[cur] {
        return cnt;
    }

    let cnt = rec(aa, aa[cur], counts_each_node) + 1;
    counts_each_node[cur] = Some(cnt);

    cnt
}

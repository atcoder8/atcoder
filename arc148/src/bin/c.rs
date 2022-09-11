// unfinished

use std::collections::VecDeque;

use im_rc::HashSet;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        pp: [Usize1; n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (i, &p) in pp.iter().enumerate() {
        graph[p].push(i + 1);
    }

    let mut depths = vec![0_usize; n];

    let mut que = VecDeque::from(vec![0]);

    while let Some(curr) = que.pop_front() {
        for &next in graph[curr].iter() {
            depths[next] = depths[curr] + 1;
            que.push_back(next);
        }
    }

    let mut stack = vec![(0_usize, false)];
    let mut in_indexes = vec![0; n];
    let mut out_indexes = vec![0; n];
    let mut idx = 0_usize;

    while let Some((curr, used_flag)) = stack.pop() {
        if used_flag {
            out_indexes[curr] = idx;
            idx += 1;
        } else {
            in_indexes[curr] = idx;
            stack.push((curr, true));
            for &next in graph[curr].iter() {
                stack.push((next, false));
            }
        }

        idx += 1;
    }

    // 出次数
    let mut deg = vec![0; n];
    for &p in pp.iter() {
        deg[p] += 1;
    }

    for _ in 0..q {
        input! {
            m: usize,
            vv: [Usize1; m],
        }

        let mut vd: Vec<(usize, usize)> = vv.iter().map(|&v| (v, depths[v])).collect();
        vd.sort_unstable_by_key(|x| x.1);

        let mut set = HashSet::new();
        for &v in vv.iter() {
            set.insert(v);
        }

        let ans = vv.iter().map(|&v| 1 + deg[v]).sum::<usize>() - (m - 1);
        println!("{}", ans);
    }
}

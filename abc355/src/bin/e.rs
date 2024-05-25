use std::{collections::VecDeque, io::Write};

use proconio::input_interactive;

const MOD: usize = 100;

fn main() {
    input_interactive!((n, l, r): (usize, usize, usize));

    let mut raised = vec![1_usize; n + 1];
    for i in 0..n {
        raised[i + 1] = 2 * raised[i];
    }

    let mut graph = vec![vec![]; raised[n] + 1];
    for exp in 0..=n {
        for left in (0..raised[n]).step_by(raised[exp]) {
            let right = left + raised[exp];
            graph[left].push(right);
            graph[right].push(left);
        }
    }

    let mut parents = vec![None; raised[n] + 1];
    let mut queue = VecDeque::from([(l, l)]);

    while let Some((par, cur)) = queue.pop_front() {
        if parents[cur].is_some() {
            continue;
        }

        parents[cur] = Some(par);

        queue.extend(graph[cur].iter().map(|&next| (cur, next)));
    }

    let mut total_rem = 0;
    let mut cur = r + 1;

    while cur != l {
        let par = parents[cur].unwrap();
        let exp = cur.abs_diff(par).trailing_zeros() as usize;

        let add = if par < cur {
            ask_rem(exp, par >> exp)
        } else {
            MOD - ask_rem(exp, cur >> exp)
        };

        total_rem = (total_rem + add) % MOD;

        cur = par;
    }

    println!("! {}", total_rem);
}

fn ask_rem(i: usize, j: usize) -> usize {
    println!("? {} {}", i, j);
    std::io::stdout().flush().unwrap();
    input_interactive!(t: usize);

    t
}

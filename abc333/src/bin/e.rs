use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some((min_k, behaviors)) => {
            println!("{}\n{}", min_k, behaviors.iter().join(" "));
        }
        None => println!("-1"),
    }
}

fn solve() -> Option<(i64, Vec<usize>)> {
    input! {
        n: usize,
        tx: [(usize, Usize1); n],
    }

    let mut portions = vec![vec![]; n];
    let mut pt_idx = vec![None; n];
    let mut portion_num = 0;
    for (turn, &(t, x)) in enumerate(&tx) {
        if t == 1 {
            portions[x].push(turn);

            pt_idx[turn] = Some(portion_num);
            portion_num += 1;
        }
    }

    let mut behaviors = vec![0; portion_num];
    let mut imos = vec![0; n + 1];
    for (et, &(t, x)) in enumerate(&tx).rev() {
        if t == 1 {
            continue;
        }

        let pt = loop {
            match portions[x].pop() {
                Some(pt) => {
                    if pt < et {
                        break pt;
                    }
                }
                None => return None,
            }
        };

        behaviors[pt_idx[pt].unwrap()] = 1;

        imos[pt] += 1;
        imos[et] -= 1;
    }

    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    Some((*imos.iter().max().unwrap(), behaviors))
}

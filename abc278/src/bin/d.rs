use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        q: usize,
    }

    let mut bb = aa.iter().map(|&a| Some(a)).collect_vec();
    let mut changed_elements = (0..n).collect_vec();
    let mut base = 0;

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                x: usize,
            }

            while let Some(idx) = changed_elements.pop() {
                bb[idx] = None;
            }

            base = x;
        } else if query_type == 2 {
            input! {
                (i, x): (Usize1, usize),
            }

            bb[i] = Some(if let Some(b) = bb[i] { b + x } else { base + x });
            changed_elements.push(i);
        } else {
            input! {
                i: Usize1,
            }

            if let Some(b) = bb[i] {
                println!("{}", b);
            } else {
                println!("{}", base);
            }
        }
    }
}

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut pigeon_to_nest = (0..n).collect_vec();
    let mut counts = vec![1_usize; n];
    let queries = (0..q).map(|_| Query::read()).collect_vec();

    let mut cnt = 0_u32;
    for &query in &queries {
        match query {
            Query::Move { pigeon, nest } => {
                let cur_nest = pigeon_to_nest[pigeon];

                pigeon_to_nest[pigeon] = nest;

                counts[cur_nest] -= 1;
                if counts[cur_nest] == 1 {
                    cnt -= 1;
                }

                counts[nest] += 1;
                if counts[nest] == 2 {
                    cnt += 1;
                }
            }

            Query::Output => println!("{}", cnt),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Move { pigeon: usize, nest: usize },
    Output,
}

impl Query {
    fn read() -> Self {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (p, h): (Usize1, Usize1),
            }

            Self::Move { pigeon: p, nest: h }
        } else {
            Self::Output
        }
    }
}

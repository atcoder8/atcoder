use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let queries = (0..q).map(|_| Query::read()).collect_vec();
    let mut heap = BinaryHeap::<i64>::new();
    let mut offset = 0_i64;
    for query in queries {
        match query {
            Query::Plant => heap.push(-offset),
            Query::Grow(t) => offset += t,
            Query::Harvest(h) => {
                let mut cnt = 0_usize;
                while let Some(&shifted_height) = heap.peek() {
                    let height = shifted_height + offset;
                    if height >= h {
                        cnt += 1;
                        heap.pop();
                    } else {
                        break;
                    }
                }

                println!("{}", cnt);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Plant,
    Grow(i64),
    Harvest(i64),
}

impl Query {
    fn read() -> Self {
        input! {
            qt: i64,
        }

        match qt {
            1 => Query::Plant,
            2 => {
                input! {
                    t: i64,
                }

                Query::Grow(t)
            }
            3 => {
                input! {
                    h: i64,
                }

                Query::Harvest(h)
            }
            _ => panic!(),
        }
    }
}

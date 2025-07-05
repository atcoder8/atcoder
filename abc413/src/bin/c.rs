use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let queries = (0..q).map(|_| Query::read());
    let mut queue = VecDeque::<(usize, usize)>::new();
    for query in queries {
        match query {
            Query::Push { c, x } => queue.push_back((c, x)),
            Query::Pop(k) => {
                let mut sum = 0_usize;
                let mut rem = k;
                while rem != 0 {
                    let (c, x) = queue.front_mut().unwrap();
                    let take = rem.min(*c);
                    sum += *x * take;
                    rem -= take;
                    *c -= take;

                    if *c == 0 {
                        queue.pop_front();
                    }
                }

                println!("{}", sum);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Push { c: usize, x: usize },
    Pop(usize),
}

impl Query {
    fn read() -> Self {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (c, x): (usize, usize),
            }

            Query::Push { c, x }
        } else {
            input! {
                k: usize,
            }

            Query::Pop(k)
        }
    }
}

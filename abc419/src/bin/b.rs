use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let queries = (0..q).map(|_| Query::read());
    let mut balls = vec![];
    for query in queries {
        match query {
            Query::In(x) => balls.push(x),
            Query::Out => {
                let pos = balls.iter().position_min().unwrap();
                let smallest = balls.remove(pos);
                println!("{}", smallest);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    In(usize),
    Out,
}

impl Query {
    fn read() -> Self {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                x: usize,
            }

            Self::In(x)
        } else {
            Self::Out
        }
    }
}

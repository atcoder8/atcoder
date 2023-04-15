use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut boxes = vec![vec![]; n + 1];
    let mut cards = vec![vec![]; 200_001_usize];

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        match query_type {
            1 => {
                input! {
                    (i, j): (usize, usize),
                }

                boxes[j].push(i);
                cards[i].push(j);
            }
            2 => {
                input! {
                    i: usize,
                }

                boxes[i].sort_unstable();
                println!("{}", join(boxes[i].iter().map(|&card| card), " "));
            }
            3 => {
                input! {
                    i: usize,
                }

                cards[i].sort_unstable();
                cards[i].dedup();
                println!("{}", join(&cards[i], " "));
            }
            _ => panic!(),
        }
    }
}

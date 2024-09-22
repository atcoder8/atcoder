use std::{cmp::Reverse, collections::BinaryHeap, io::Write};

use itertools::Itertools;
use proconio::input_interactive;

const N: usize = 1000;
const M: usize = 10;
const Q: usize = 950;

fn main() {
    input_interactive!((_n, _m, _q): (usize, usize, usize));

    let mut query = Query::new();

    let mut same_coins = vec![0];
    let mut diff_coins = vec![];
    let mut coin = 1;
    while same_coins.len() <= M && diff_coins.len() <= M {
        if query.ask_same(0, coin) {
            same_coins.push(coin);
        } else {
            diff_coins.push(coin);
        }

        coin += 1;
    }

    let (real_coins, mut fake_coins) = if same_coins.len() > M {
        (same_coins, diff_coins)
    } else {
        (diff_coins, same_coins)
    };
    let real_coin = real_coins[0];

    let mut heap: BinaryHeap<(Reverse<usize>, Vec<usize>)> =
        (coin..N).map(|i| (Reverse(1), vec![i])).collect();
    let mut next_heap = BinaryHeap::new();
    while !heap.is_empty() {
        while heap.len() >= 2 {
            let (_, mut chunk1) = heap.pop().unwrap();
            let (_, mut chunk2) = heap.pop().unwrap();
            if query.ask_same(chunk1[0], chunk2[0]) {
                chunk1.append(&mut chunk2);
                let merged_len = chunk1.len();
                if merged_len <= M {
                    next_heap.push((Reverse(merged_len), chunk1));
                }
            } else {
                for chunk in [chunk1, chunk2] {
                    if !query.ask_same(chunk[0], real_coin) {
                        fake_coins.extend(chunk);
                    }
                }
            }
        }
        if let Some((_, chunk)) = heap.pop() {
            if !query.ask_same(chunk[0], real_coin) {
                fake_coins.extend(chunk);
            }
        }

        std::mem::swap(&mut heap, &mut next_heap);
    }

    fake_coins.sort_unstable();

    query.output_answer(&fake_coins);
}

struct Query {
    count_queries: usize,
}

impl Query {
    fn new() -> Self {
        Self { count_queries: 0 }
    }

    fn ask_same(&mut self, x: usize, y: usize) -> bool {
        assert!(self.count_queries < Q);

        self.count_queries += 1;

        println!("? {} {}", x + 1, y + 1);
        std::io::stdout().flush().unwrap();

        input_interactive!(response: String);

        match response.as_str() {
            "0" => true,
            "1" => false,
            _ => panic!(),
        }
    }

    fn output_answer(&self, fake_coins: &[usize]) {
        assert_eq!(fake_coins.len(), M);

        println!("! {}", fake_coins.iter().map(|i| i + 1).join(" "));
        std::io::stdout().flush().unwrap();
    }
}

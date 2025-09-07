use itertools::Itertools;
use proconio::input;
use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::stopwatch::Stopwatch;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut rng = rand::thread_rng();

    match solve(&mut rng, &xy) {
        Some((a, b, c)) => println!("Yes\n{a} {b} {c}"),
        None => println!("No"),
    }
}

fn solve(rng: &mut ThreadRng, xy: &[(i64, i64)]) -> Option<(i64, i64, i64)> {
    let is_ok = |a: i64, b: i64, c: i64| {
        xy.iter()
            .filter(move |&&(x, y)| a * x + b * y + c == 0)
            .count()
            > xy.len() / 2
    };

    let stop_watch = Stopwatch::start();
    while stop_watch.elapsed_time() < 1.8 {
        let (&(x1, y1), &(x2, y2)) = xy.choose_multiple(rng, 2).collect_tuple().unwrap();
        let dx = x1 - x2;
        let dy = y1 - y2;
        let (a, b, c) = (dy, -dx, dx * y1 - dy * x1);
        if is_ok(a, b, c) {
            return Some((a, b, c));
        }
    }

    None
}

pub mod stopwatch {
    use std::time::Instant;

    #[derive(Debug, Clone, Copy)]
    pub struct Stopwatch {
        start_instant: Instant,
    }

    impl Stopwatch {
        pub fn start() -> Self {
            Self {
                start_instant: Instant::now(),
            }
        }

        pub fn elapsed_time(&self) -> f64 {
            let duration = self.start_instant.elapsed();
            duration.as_secs_f64()
        }
    }
}

use itertools::{izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve().iter().join(" ")).join("\n");
    println!("{output}");
}

#[derive(Debug, Clone, Copy)]
struct DirectionCounter {
    direction: char,
    count: usize,
}

impl DirectionCounter {
    fn push_direction(&mut self, direction: char) {
        if direction == self.direction {
            self.count += 1;
        } else {
            *self = Self {
                direction,
                count: 1,
            }
        }
    }
}

fn solve() -> Vec<i64> {
    input! {
        n: usize,
        s: Chars,
    }

    let mut offset_left = vec![0_usize; n];
    let mut offset_right = vec![0_usize; n];

    let mut counter = DirectionCounter {
        direction: 'L',
        count: 0,
    };
    for i in 0..n {
        if counter.direction == 'R' {
            offset_left[i] += counter.count;
        } else {
            offset_right[i] += counter.count;
        }

        if i < n - 1 {
            counter.push_direction(s[i]);
        }
    }

    let mut counter = DirectionCounter {
        direction: 'L',
        count: 0,
    };
    for i in (0..n).rev() {
        if counter.direction == 'L' {
            offset_left[i] += counter.count;
        } else {
            offset_right[i] += counter.count;
        }

        if i > 0 {
            counter.push_direction(s[i - 1]);
        }
    }

    let mut imos = vec![0_i64; n + 1];
    for (&offset_left, &offset_right) in izip!(&offset_left, &offset_right) {
        imos[offset_left] += 1;
        imos[n - offset_right] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    imos.pop();
    imos
}

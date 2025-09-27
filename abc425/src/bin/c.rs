use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [usize; n],
    }

    let queries = (0..q).map(|_| Query::read());

    let mut prefix_sum = aa.clone();
    for i in 0..n - 1 {
        prefix_sum[i + 1] += prefix_sum[i];
    }
    prefix_sum.insert(0, 0);

    let calc_sum = |l: usize, r: usize| prefix_sum[r] - prefix_sum[l];

    let mut offset = 0_usize;

    for query in queries {
        match query {
            Query::Move(c) => offset += c,
            Query::Output { l, r } => {
                let diff = r - l;
                let left = (l + offset) % n;
                let sum = if left + diff <= n {
                    calc_sum(left, left + diff)
                } else {
                    calc_sum(left, n) + calc_sum(0, (left + diff) - n)
                };
                println!("{sum}");
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Query {
    Move(usize),
    Output { l: usize, r: usize },
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        if qt == 1 {
            input! {
                c: usize,
            }

            Self::Move(c)
        } else {
            input! {
                (l, r): (Usize1, usize),
            }

            Self::Output { l, r }
        }
    }
}

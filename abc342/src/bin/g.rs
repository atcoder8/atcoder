use std::collections::BTreeSet;

use itertools::{enumerate, Itertools};
use maplit::btreeset;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        q: usize,
    }

    #[derive(Debug, Clone, Copy)]
    enum Query {
        Add { l: usize, r: usize, x: usize },

        Remove(usize),

        Output(usize),
    }

    impl Query {
        fn new() -> Self {
            input! {
                qt: usize,
            }

            match qt {
                1 => {
                    input! {
                        (l, r, x): (Usize1, usize, usize),
                    }

                    Query::Add { l, r, x }
                }

                2 => {
                    input! {
                        i: Usize1,
                    }

                    Query::Remove(i)
                }

                3 => {
                    input! {
                        i: Usize1,
                    }

                    Query::Output(i)
                }

                _ => unreachable!(),
            }
        }
    }

    let queries = (0..q).map(|_| Query::new()).collect_vec();

    let mut st = Segtree::from(aa);

    let mut ans = vec![];
    for (qi, &query) in enumerate(&queries) {
        match query {
            Query::Add { l, r, x } => st.apply(l, r, (x, qi), true),

            Query::Remove(remove_idx) => {
                let Query::Add { l, r, x } = queries[remove_idx] else { panic!() };
                st.apply(l, r, (x, remove_idx), false);
            }

            Query::Output(output_idx) => ans.push(st.get(output_idx)),
        }
    }

    println!("{}", ans.iter().join("\n"));
}

pub struct Segtree {
    size: usize,
    init_seq: Vec<usize>,
    chmax_tree: Vec<BTreeSet<(usize, usize)>>,
}

impl From<Vec<usize>> for Segtree {
    fn from(init_seq: Vec<usize>) -> Self {
        let log = (0..).find(|&i| (init_seq.len() - 1) >> i == 0).unwrap();
        let size = 1 << log;

        Self {
            size,
            init_seq,
            chmax_tree: vec![btreeset! {}; 2 * size],
        }
    }
}

impl Segtree {
    fn apply(&mut self, left: usize, right: usize, v: (usize, usize), insert: bool) {
        let mut left = left + self.size;
        let mut right = right + self.size;

        while left < right {
            if left & 1 == 1 {
                if insert {
                    self.chmax_tree[left].insert(v);
                } else {
                    self.chmax_tree[left].remove(&v);
                }

                left += 1;
            }

            if right & 1 == 1 {
                right -= 1;

                if insert {
                    self.chmax_tree[right].insert(v);
                } else {
                    self.chmax_tree[right].remove(&v);
                }
            }

            left >>= 1;
            right >>= 1;
        }
    }

    fn get(&self, p: usize) -> usize {
        let mut max = self.init_seq[p];
        let mut p = p + self.size;

        while p != 0 {
            max = max.max(self.chmax_tree[p].last().map_or(0, |v| v.0));
            p >>= 1;
        }

        max
    }
}

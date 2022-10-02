use proconio::input;

use crate::atcoder8_library::lis::StronglyLIS;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    }

    let mut ab: Vec<(usize, usize)> = aa.iter().zip(bb.iter()).map(|(&a, &b)| (a, b)).collect();
    ab.sort_unstable_by_key(|x| x.0);

    let sorted_bb: Vec<usize> = ab.iter().map(|x| x.1).collect();

    let lis = StronglyLIS::from(sorted_bb);
    let mut ans = n + lis.lis_len();

    ab.sort_unstable_by_key(|x| x.1);

    let sorted_aa: Vec<usize> = ab.iter().map(|x| x.0).collect();
    let lis = StronglyLIS::from(sorted_aa);
    ans = ans.max(n + lis.lis_len());

    println!("{}", ans);
}

pub mod atcoder8_library {
    pub mod lis {
        use superslice::Ext;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            dp: Vec<T>,
        }

        impl<T> WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            pub fn new() -> Self {
                Self { dp: vec![] }
            }

            pub fn push(&mut self, x: T) {
                let idx = self.dp.upper_bound(&x);
                if idx < self.dp.len() {
                    self.dp[idx] = x;
                } else {
                    self.dp.push(x);
                }
            }

            pub fn lis_len(&self) -> usize {
                self.dp.len()
            }
        }

        impl<T> Default for WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            fn default() -> Self {
                WeeklyLIS::new()
            }
        }

        impl<T> From<Vec<T>> for WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            fn from(seq: Vec<T>) -> Self {
                let mut lis = Self::default();
                for x in seq {
                    lis.push(x);
                }
                lis
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            dp: Vec<T>,
        }

        impl<T> StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            pub fn new() -> Self {
                Self { dp: vec![] }
            }

            pub fn push(&mut self, x: T) {
                let idx = self.dp.lower_bound(&x);
                if idx < self.dp.len() {
                    self.dp[idx] = x;
                } else {
                    self.dp.push(x);
                }
            }

            pub fn lis_len(&self) -> usize {
                self.dp.len()
            }
        }

        impl<T> Default for StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            fn default() -> Self {
                StronglyLIS::new()
            }
        }

        impl<T> From<Vec<T>> for StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            fn from(seq: Vec<T>) -> Self {
                let mut lis = Self::default();
                for x in seq {
                    lis.push(x);
                }
                lis
            }
        }
    }
}

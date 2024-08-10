use cumulative_sum::CumulativeSum;
use itertools::Itertools;
use proconio::input;

const OFFSET: i64 = 3 * 10_i64.pow(6);
const MAX: usize = 2 * OFFSET as usize;

fn main() {
    input! {
        (n, d): (usize, usize),
        xy: [(i64, i64); n],
    }

    let xy = xy
        .into_iter()
        .map(|(x, y)| ((x + OFFSET) as usize, (y + OFFSET) as usize))
        .collect_vec();

    let med_y = xy
        .iter()
        .map(|&(_, y)| y)
        .sorted_unstable()
        .nth(n / 2)
        .unwrap();

    let mut hor_counts = vec![0_usize; MAX + 1];
    let mut hor_weighted = vec![0_usize; MAX + 1];
    let mut ver_counts = vec![0_usize; MAX + 1];
    let mut ver_weighted = vec![0_usize; MAX + 1];
    for &(x, y) in &xy {
        hor_counts[x] += 1;
        hor_weighted[x] += MAX - x;
        ver_counts[y] += 1;
        ver_weighted[y] += MAX - y;
    }

    let acc_hor_count = CumulativeSum::from(hor_counts);
    let acc_hor_weighted = CumulativeSum::from(hor_weighted);
    let acc_ver_count = CumulativeSum::from(ver_counts);
    let acc_ver_weighted = CumulativeSum::from(ver_weighted);

    let calc_sum_dist = |x: usize, y: usize| {
        let left_sum = acc_hor_weighted.sum(0..x) - (MAX - x) * acc_hor_count.sum(0..x);
        let right_sum = (MAX - x) * acc_hor_count.sum(x + 1..) - acc_hor_weighted.sum(x + 1..);
        let upper_sum = acc_ver_weighted.sum(0..y) - (MAX - y) * acc_ver_count.sum(0..y);
        let lower_sum = (MAX - y) * acc_ver_count.sum(y + 1..) - acc_ver_weighted.sum(y + 1..);

        left_sum + right_sum + upper_sum + lower_sum
    };

    let is_ok = |x: usize, y: usize| calc_sum_dist(x, y) <= d;

    // 各xについて条件を満たすyの範囲を求める
    // 中央値の上下をそれぞれ二分探索
    let count = |x: usize| -> usize {
        if !is_ok(x, med_y) {
            return 0;
        }

        let mut upper_ok = med_y;
        let mut upper_ng = MAX;

        while upper_ok.abs_diff(upper_ng) > 1 {
            let mid = (upper_ok + upper_ng) / 2;

            if is_ok(x, mid) {
                upper_ok = mid;
            } else {
                upper_ng = mid;
            }
        }

        let mut lower_ok = med_y;
        let mut lower_ng = 0;

        while lower_ok.abs_diff(lower_ng) > 1 {
            let mid = (lower_ok + lower_ng) / 2;

            if is_ok(x, mid) {
                lower_ok = mid;
            } else {
                lower_ng = mid;
            }
        }

        upper_ok - lower_ok + 1
    };

    let ans = (0..=MAX).map(count).sum::<usize>();
    println!("{}", ans);
}

pub mod cumulative_sum {
    //! Calculates the interval sum of a numerical sequence
    //! in constant time using cumulative sum.

    use std::ops::{Add, RangeBounds, Sub};

    use num::Zero;

    /// Calculates the interval sum of a numerical sequence using cumulative sum.
    ///
    /// # Examples
    /// ```
    /// use atcoder8_library::cumulative_sum::CumulativeSum;
    ///
    /// let cumsum = CumulativeSum::from(vec![3, -1, 4, -1, -5, 9, 2]);
    /// assert_eq!(cumsum.sum(2..6), 7);
    /// ```
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct CumulativeSum<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
    {
        /// Cumulative sum of sequence
        cumsum: Vec<T>,
    }

    impl<T> CumulativeSum<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
    {
        /// Calculates cumulative sum for an empty sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let cumsum = CumulativeSum::<i32>::new();
        /// assert_eq!(cumsum.sum(..), 0);
        /// ```
        pub fn new() -> Self {
            Self {
                cumsum: vec![T::zero()],
            }
        }

        /// Returns the length of a sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.seq_len(), 3);
        /// ```
        pub fn seq_len(&self) -> usize {
            self.cumsum.len() - 1
        }

        /// Checks if the sequence is empty.
        ///
        /// # Examples
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let mut cumsum = CumulativeSum::from(vec![3]);
        /// assert!(!cumsum.is_empty());
        /// cumsum.pop();
        /// assert!(cumsum.is_empty());
        /// ```
        pub fn is_empty(&self) -> bool {
            self.seq_len().is_zero()
        }

        /// Adds an element to the end of the sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let mut cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.sum(1..), 3);
        /// cumsum.push(10);
        /// assert_eq!(cumsum.sum(1..), 13);
        /// ```
        pub fn push(&mut self, x: T) {
            self.cumsum.push(self.cumsum.last().unwrap().clone() + x);
        }

        /// Removes the last element of a sequence and returns the end of the cumulative sum.
        ///
        /// # Panics
        ///
        /// Panics if the sequence is empty.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let mut cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.sum(1..), 3);
        /// assert_eq!(cumsum.pop(), 6);
        /// assert_eq!(cumsum.sum(1..), -1);
        /// ```
        pub fn pop(&mut self) -> T {
            assert!(!self.is_empty(), "The sequence is empty.");

            self.cumsum.pop().unwrap()
        }

        /// Calculates the sum of intervals.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let seq = vec![3, -1, 4, -1, -5, 9, 2];
        /// let cumsum = CumulativeSum::from(seq.clone());
        ///
        /// assert_eq!(cumsum.sum(..), seq.iter().sum());
        /// assert_eq!(cumsum.sum(2..), seq[2..].iter().sum());
        /// assert_eq!(cumsum.sum(..5), seq[..5].iter().sum());
        /// assert_eq!(cumsum.sum(2..5), seq[2..5].iter().sum());
        /// assert_eq!(cumsum.sum(2..2), 0);
        /// ```
        pub fn sum<R>(&self, rng: R) -> T
        where
            R: RangeBounds<usize>,
        {
            let l = match rng.start_bound() {
                std::ops::Bound::Included(&start_bound) => start_bound,
                std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let r = match rng.end_bound() {
                std::ops::Bound::Included(&end_bound) => end_bound + 1,
                std::ops::Bound::Excluded(&end_bound) => end_bound,
                std::ops::Bound::Unbounded => self.cumsum.len() - 1,
            };

            // The end of the range must be after the starting point.
            assert!(l <= r, "slice index starts at {} but ends at {}", l, r);

            self.cumsum[r].clone() - self.cumsum[l].clone()
        }

        /// Converts the cumulative sum to `Vec<T>`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.to_vec(), vec![0, 3, 2, 6]);
        /// ```
        pub fn to_vec(self) -> Vec<T>
        where
            T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
        {
            self.cumsum
        }
    }

    impl<T> From<Vec<T>> for CumulativeSum<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
    {
        fn from(seq: Vec<T>) -> Self {
            let mut accumulate = vec![T::zero()];
            accumulate.reserve(accumulate.len() - 1);
            for (i, x) in seq.into_iter().enumerate() {
                accumulate.push(accumulate[i].clone() + x);
            }
            Self { cumsum: accumulate }
        }
    }
}

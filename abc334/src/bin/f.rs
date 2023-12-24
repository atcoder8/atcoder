use itertools::Itertools;
use proconio::input;
use segment_tree::Monoid;

use crate::segment_tree::SegmentTree;

fn main() {
    input! {
        (n, k): (usize, usize),
        start_coord: (f64, f64),
        mut xy: [(f64, f64); n],
    }

    xy.push(start_coord);
    let dists = [start_coord]
        .iter()
        .chain(&xy)
        .tuple_windows()
        .map(|(&coord1, &coord2)| calc_dist(coord1, coord2))
        .collect_vec();

    let mut segtree = SegmentTree::<S>::new(n + 1);
    segtree.set(0, S(0.0));
    for i in 1..=n {
        let prev_sum_diff_dist = segtree.prod(i.saturating_sub(k)..i).0;
        let via_dist = calc_dist(xy[i - 1], start_coord) + calc_dist(start_coord, xy[i]);
        let diff_dist = via_dist - dists[i];
        segtree.set(i, S(prev_sum_diff_dist + diff_dist));
    }

    let ans = dists.iter().sum::<f64>() + segtree.get(n).0;
    println!("{}", ans);
}

fn calc_dist(coord1: (f64, f64), coord2: (f64, f64)) -> f64 {
    let diff_x = coord1.0 - coord2.0;
    let diff_y = coord1.1 - coord2.1;

    diff_x.hypot(diff_y)
}

#[derive(Debug, Clone, Copy)]
struct S(f64);

impl Monoid for S {
    fn e() -> Self {
        Self(std::f64::INFINITY)
    }

    fn op(a: Self, b: Self) -> Self {
        S(a.0.min(b.0))
    }
}

pub mod segment_tree {
    //! Performs the following operations on a number sequence of length `n`
    //! consisting of elements of monoid in logarithmic time of `n`.
    //! * Updates the specified element
    //! * Calculates the product of elements in the specified range.

    use std::ops::RangeBounds;

    /// Defines the method signature of the monoid.
    pub trait Monoid: Clone {
        /// The identity element
        fn e() -> Self;

        /// The binary operation
        fn op(a: Self, b: Self) -> Self;
    }

    /// # Examples
    ///
    /// ```
    /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
    ///
    /// #[derive(Debug, Clone, PartialEq)]
    /// struct Data(i32);
    ///
    /// impl Monoid for Data {
    ///     fn e() -> Self {
    ///         Data(0)
    ///     }
    ///
    ///     fn op(a: Self, b: Self) -> Self {
    ///         Data(a.0.max(b.0))
    ///     }
    /// }
    ///
    /// let seq = vec![Data(3), Data(-1), Data(4), Data(1), Data(-5), Data(9)];
    /// let segtree = SegmentTree::from(seq);
    /// assert_eq!(segtree.prod(1..5), Data(4));
    /// ```
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SegmentTree<M>
    where
        M: Monoid,
    {
        /// Length of sequence
        n: usize,

        /// Sequences and intermediate data
        data: Vec<M>,
    }

    impl<M: Monoid> From<Vec<M>> for SegmentTree<M> {
        fn from(seq: Vec<M>) -> Self {
            let mut segtree = Self::new(seq.len());
            for (i, x) in seq.into_iter().enumerate() {
                segtree.set(i, x);
            }
            segtree
        }
    }

    impl<M: Monoid> SegmentTree<M> {
        /// Creates a Segment Tree for a sequence of length `n`.
        /// All elements of the sequence are initialized with the identity element.
        ///
        /// # Arguments
        ///
        /// * `n` - Length of sequence
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        ///
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn e() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn op(a: Self, b: Self) -> Self {
        ///         Data(a.0.max(b.0))
        ///     }
        /// }
        ///
        /// let segtree = SegmentTree::<Data>::new(5);
        /// assert_eq!(segtree.get(2), Data(0));
        /// ```
        pub fn new(n: usize) -> Self {
            let mut data_len = 1;
            while data_len < n {
                data_len *= 2;
            }
            data_len *= 2;

            Self {
                n,
                data: vec![M::e(); data_len],
            }
        }

        /// Update the `p`-th number in the sequence to `x`.
        ///
        /// # Arguments
        ///
        /// * `p` - Index of the element to update
        /// * `x` - Value to assign
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        ///
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn e() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn op(a: Self, b: Self) -> Self {
        ///         Data(a.0.max(b.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4)];
        /// let mut segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.get(1), Data(-1));
        ///
        /// segtree.set(1, Data(10));
        /// assert_eq!(segtree.get(1), Data(10));
        /// ```
        pub fn set(&mut self, p: usize, x: M) {
            assert!(
            p < self.n,
            "The specified index {} is outside the range of the sequence; the length of the sequence is {}.",
            p,
            self.n,
        );

            let mut p = p + self.data.len() / 2;
            self.data[p] = x;
            while p != 1 {
                p >>= 1;
                self.data[p] = M::op(self.data[2 * p].clone(), self.data[2 * p + 1].clone());
            }
        }

        /// Returns the `p`-th element.
        ///
        /// # Arguments
        ///
        /// * `p` - Index of the element to get
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        ///
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn e() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn op(a: Self, b: Self) -> Self {
        ///         Data(a.0.max(b.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4)];
        /// let segtree = SegmentTree::from(seq);
        /// assert_eq!(segtree.get(1), Data(-1));
        /// ```
        pub fn get(&self, p: usize) -> M {
            assert!(
            p < self.n,
            "The specified index {} is outside the range of the sequence; the length of the sequence is {}.",
            p,
            self.n,
        );

            self.data[self.data.len() / 2 + p].clone()
        }

        /// Calculates the product of elements of the sequence in the specified range.
        ///
        /// # Arguments
        ///
        /// * `rng` - Range of the product
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        ///
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn e() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn op(a: Self, b: Self) -> Self {
        ///         Data(a.0.max(b.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4), Data(1), Data(-5), Data(9)];
        /// let segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.prod(..), Data(9));
        /// assert_eq!(segtree.prod(2..), Data(9));
        /// assert_eq!(segtree.prod(..5), Data(4));
        /// assert_eq!(segtree.prod(2..5), Data(4));
        /// assert_eq!(segtree.prod(2..2), Data(0));
        /// ```
        pub fn prod<R>(&self, rng: R) -> M
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
                std::ops::Bound::Unbounded => self.n,
            };

            assert!(l <= r, "The slice index starts at {} but ends at {}", l, r);
            assert!(
            r <= self.n,
            "The specified range {}..{} is outside the range of the sequence; the length of sequence is {}",
            l,
            r,
            self.n,
        );

            let mut sml = M::e();
            let mut smr = M::e();

            let mut l = l + self.data.len() / 2;
            let mut r = r + self.data.len() / 2;

            while l < r {
                if l & 1 != 0 {
                    sml = M::op(sml, self.data[l].clone());
                    l += 1;
                }

                if r & 1 != 0 {
                    r -= 1;
                    smr = M::op(self.data[r].clone(), smr);
                }

                l >>= 1;
                r >>= 1;
            }

            M::op(sml, smr)
        }

        /// Returns the product of all elements of a sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        ///
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn e() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn op(a: Self, b: Self) -> Self {
        ///         Data(a.0.max(b.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4), Data(1), Data(-5)];
        /// let segtree = SegmentTree::from(seq);
        /// assert_eq!(segtree.all_prod(), Data(4));
        /// ```
        pub fn all_prod(&self) -> M {
            self.data[1].clone()
        }

        /// Performs a binary search on the segment tree.
        ///
        /// Returns one `r` satisfying
        /// `f(op(seq[l], seq[l + 1], ... , seq[r - 1])) == true` and
        /// `f(op(seq[l], seq[l + 1], ... , seq[r])) == false`.
        ///
        /// If no such `r` exists, returns the maximum index of the sequence plus 1;
        /// if the length of the sequence is `n`, then `n` is returned.
        ///
        /// # Arguments
        ///
        /// * `l` - Left boundary of the range of the sequence
        /// * `f` - Mapping from any element of a monoid to a bool value
        ///
        /// # Panics
        ///
        /// Panic if any of the following:
        /// * The identity element is mapped to `false` by `f`.
        /// * The left boundary `l` is outside the range of the sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        ///
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(usize);
        ///
        /// impl Monoid for Data {
        ///     fn e() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn op(a: Self, b: Self) -> Self {
        ///         Data(a.0.max(b.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(1), Data(4), Data(1), Data(5), Data(9)];
        /// let segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.bs_right_boundary(1, |&Data(x)| x < 1), 1);
        /// assert_eq!(segtree.bs_right_boundary(1, |&Data(x)| x < 5), 4);
        /// assert_eq!(segtree.bs_right_boundary(1, |&Data(x)| x < 10), 6);
        /// ```
        pub fn bs_right_boundary<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&M) -> bool,
        {
            assert!(
                f(&M::e()),
                "The identity element must be mapped to true by `f`."
            );

            assert!(
            l <= self.n,
            "Left boundary {} is outside the range of the sequence; the length of sequence is {}.",
            l,
            self.n,
        );

            if l == self.n {
                return self.n;
            }

            let size = self.data.len() / 2;

            let mut l = l + size;
            let mut sm = M::e();

            loop {
                while l % 2 == 0 {
                    l >>= 1;
                }

                if !f(&M::op(sm.clone(), self.data[l].clone())) {
                    while l < size {
                        l *= 2;
                        let res = M::op(sm.clone(), self.data[l].clone());
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }

                    return l - size;
                }

                sm = M::op(sm, self.data[l].clone());
                l += 1;

                if l & l.wrapping_neg() == l {
                    break;
                }
            }

            self.n
        }

        /// Performs a binary search on the segment tree.
        ///
        /// Returns one `l` satisfying
        /// `f(op(seq[l], seq[l + 1], ... , seq[r - 1])) == true` and
        /// `f(op(seq[l - 1], seq[l + 1], ... , seq[r - 1])) == false`.
        ///
        /// If no such `l` exists, returns `0`.
        ///
        /// # Arguments
        ///
        /// * `r` - Right boundary of the range of the sequence
        /// * `f` - Mapping from any element of a monoid to a bool value
        ///
        /// # Panics
        ///
        /// Panic if any of the following:
        /// * The identity element is mapped to `false` by `f`.
        /// * The right boundary `r` is outside the range of the sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        ///
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(usize);
        ///
        /// impl Monoid for Data {
        ///     fn e() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn op(a: Self, b: Self) -> Self {
        ///         Data(a.0.max(b.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(1), Data(4), Data(1), Data(5), Data(9)];
        /// let segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.bs_left_boundary(4, |&Data(x)| x < 1), 4);
        /// assert_eq!(segtree.bs_left_boundary(4, |&Data(x)| x < 3), 3);
        /// assert_eq!(segtree.bs_left_boundary(4, |&Data(x)| x < 5), 0);
        /// ```
        pub fn bs_left_boundary<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&M) -> bool,
        {
            assert!(
                f(&M::e()),
                "The identity element must be mapped to true by `f`."
            );

            assert!(
            r <= self.n,
            "Right boundary {} is outside the range of the sequence; the length of sequence is {}.",
            r,
            self.n,
        );

            if r == 0 {
                return 0;
            }

            let size = self.data.len() / 2;

            let mut r = r + size;
            let mut sm = M::e();

            loop {
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }

                if !f(&M::op(self.data[r].clone(), sm.clone())) {
                    while r < size {
                        r = 2 * r + 1;
                        let res = M::op(self.data[r].clone(), sm.clone());
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }

                    return r + 1 - size;
                }

                sm = M::op(self.data[r].clone(), sm);

                if r & r.wrapping_neg() == r {
                    break;
                }
            }

            0
        }
    }
}

use std::collections::BTreeMap;

use itertools::Itertools;
use modint2::Modint998244353;
use proconio::input;
use segment_tree::Monoid;
use superslice::Ext;

use crate::segment_tree::SegmentTree;

type Mint = Modint998244353;

fn main() {
    input! {
        (n, h): (usize, usize),
        xx: [usize; n],
    }

    let mut sorted_xx = xx.clone();
    sorted_xx.sort_unstable();

    let mut ccs = vec![];
    let mut left = 0;
    while left < n {
        let right = (left + 1..n)
            .find(|&right| sorted_xx[right] - sorted_xx[right - 1] > h)
            .unwrap_or(n);
        ccs.push(ConnectedComponent::new(&sorted_xx[left..right]));

        left = right;
    }

    let mut st = SegmentTree::from(vec![S(Mint::new(0)); ccs.len()]);

    let probabilities = xx
        .iter()
        .map(|&x| {
            let cc_idx = ccs.upper_bound_by_key(&x, |cc| cc.coords[0]) - 1;
            let prob = ccs[cc_idx].push(x);
            let updated_prob = st.get(cc_idx).0 + prob;
            st.set(cc_idx, S(updated_prob));

            st.prod(0..cc_idx).0 * st.prod(cc_idx + 1..).0 * prob
        })
        .collect_vec();

    let raised = Mint::new(2).pow(n);
    let ans = probabilities.iter().map(|&prob| prob * raised).join(" ");
    println!("{}", ans);
}

struct ConnectedComponent {
    coords: Vec<usize>,
    left_counts: BTreeMap<usize, usize>,
    right_counts: BTreeMap<usize, usize>,
}

impl ConnectedComponent {
    fn new(coords: &[usize]) -> Self {
        Self {
            coords: coords.to_owned(),
            left_counts: BTreeMap::new(),
            right_counts: BTreeMap::new(),
        }
    }

    fn push(&mut self, x: usize) -> Mint {
        let left = self.left_counts.range(..x).next_back();
        let right = self.right_counts.range(x + 1..).next();

        let left_cnt = left.map(|(_, &cnt)| cnt).unwrap_or(0);
        let right_cnt = right.map(|(_, &cnt)| cnt).unwrap_or(0);

        let newly_prob = Mint::new(2).inv().pow(left_cnt + right_cnt);

        let pos = self.coords.binary_search(&x).unwrap();

        let right_ok = pos == 0 || Some(self.coords[pos - 1]) == left.map(|(&x, _)| x);
        let left_ok =
            pos == self.coords.len() - 1 || Some(self.coords[pos + 1]) == right.map(|(&x, _)| x);

        self.left_counts.insert(x, left_cnt + 1);
        self.right_counts.insert(x, right_cnt + 1);

        newly_prob / 2 * (right_ok as usize + left_ok as usize)
    }
}

#[derive(Debug, Clone, Copy)]
struct S(Mint);

impl Monoid for S {
    fn id() -> Self {
        Self(Mint::new(1))
    }

    fn product(&self, rhs: &Self) -> Self {
        Self(self.0 * rhs.0)
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
        fn id() -> Self;

        /// The binary operation
        fn product(&self, rhs: &Self) -> Self;
    }

    /// # Examples
    ///
    /// ```
    /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
    /// #
    /// #[derive(Debug, Clone, PartialEq)]
    /// struct Data(i32);
    ///
    /// impl Monoid for Data {
    ///     fn id() -> Self {
    ///         Data(0)
    ///     }
    ///
    ///     fn prod(&self, rhs: &Self) -> Self {
    ///         Data(self.0.max(rhs.0))
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
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
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
                data: vec![M::id(); data_len],
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
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
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
                self.data[p] = self.data[2 * p].product(&self.data[2 * p + 1]);
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
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
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
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
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

            let mut sml = M::id();
            let mut smr = M::id();

            let mut l = l + self.data.len() / 2;
            let mut r = r + self.data.len() / 2;

            while l < r {
                if l & 1 != 0 {
                    sml = sml.product(&self.data[l]);
                    l += 1;
                }

                if r & 1 != 0 {
                    r -= 1;
                    smr = self.data[r].product(&smr);
                }

                l >>= 1;
                r >>= 1;
            }

            sml.product(&smr)
        }

        /// Returns the product of all elements of a sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
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
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(usize);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
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
                f(&M::id()),
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
            let mut sm = M::id();

            loop {
                while l % 2 == 0 {
                    l >>= 1;
                }

                if !f(&sm.product(&self.data[l])) {
                    while l < size {
                        l *= 2;
                        let res = sm.product(&self.data[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }

                    return l - size;
                }

                sm = sm.product(&self.data[l]);
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
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(usize);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
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
                f(&M::id()),
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
            let mut sm = M::id();

            loop {
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }

                if !f(&self.data[r].product(&sm)) {
                    while r < size {
                        r = 2 * r + 1;
                        let res = self.data[r].product(&sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }

                    return r + 1 - size;
                }

                sm = self.data[r].product(&sm);

                if r & r.wrapping_neg() == r {
                    break;
                }
            }

            0
        }
    }
}

pub mod modint2 {
    //! This module implements modular arithmetic.

    use std::{
        iter::{Product, Sum},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    type InnerType = u32;

    /// Returns `x` such that `a * x` is equivalent to `1` with `m` as the modulus.
    fn modinv(a: u32, m: u32) -> u32 {
        let (mut a, mut b, mut s, mut t) = (a as i64, m as i64, 1, 0);
        while b != 0 {
            let q = a / b;
            a -= q * b;
            std::mem::swap(&mut a, &mut b);
            s -= q * t;
            std::mem::swap(&mut s, &mut t);
        }

        assert_eq!(
            a.abs(),
            1,
            "\
There is no multiplicative inverse of `a` with `m` as the modulus, \
because `a` and `m` are not prime to each other (gcd(a, m) = {}).",
            a.abs()
        );

        ((s % m as i64 + m as i64) % m as i64) as u32
    }

    pub trait Reminder {
        /// Returns the remainder divided by `modulus`.
        fn reminder(self, modulus: InnerType) -> InnerType;
    }

    macro_rules! impl_reminder_for_small_unsigned_int {
        ($($unsigned_small_int: tt), *) => {
            $(
                impl Reminder for $unsigned_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        self as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `u8`, `u16` and `u32`.
    impl_reminder_for_small_unsigned_int!(u8, u16, u32);

    macro_rules! impl_reminder_for_large_unsigned_int {
        ($($unsigned_large_int: tt), *) => {
            $(
                impl Reminder for $unsigned_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self) as InnerType
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `usize`, `u64` and `u128`.
    impl_reminder_for_large_unsigned_int!(usize, u64, u128);

    macro_rules! impl_reminder_for_small_signed_int {
        ($($signed_small_int: tt), *) => {
            $(
                impl Reminder for $signed_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self as i32 % modulus as i32 + modulus as i32) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `i8`, `i16` and `i32`.
    impl_reminder_for_small_signed_int!(i8, i16, i32);

    macro_rules! impl_reminder_for_large_signed_int {
        ($($signed_large_int: tt), *) => {
            $(
                impl Reminder for $signed_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self + modulus as Self) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `isize`, `i64` and `i128`.
    impl_reminder_for_large_signed_int!(isize, i64, i128);

    /// Structure for modular arithmetic.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modint<const MODULUS: InnerType> {
        rem: InnerType,
    }

    impl<const MODULUS: InnerType> std::fmt::Display for Modint<MODULUS> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.rem)
        }
    }

    impl<const MODULUS: InnerType> Default for Modint<MODULUS> {
        /// Returns a `Modint` instance equivalent to `0`.
        fn default() -> Self {
            Self::raw(0)
        }
    }

    impl<T, const MODULUS: InnerType> From<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn from(value: T) -> Self {
            Self::new(value)
        }
    }

    impl<const MODULUS: InnerType> Add<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn add(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Sub<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn sub(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + MODULUS - rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Mul<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn mul(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem as u64 * rhs.rem as u64 % MODULUS as u64) as InnerType)
        }
    }

    impl<const MODULUS: InnerType> Div<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        #[allow(clippy::suspicious_arithmetic_impl)]
        fn div(self, rhs: Modint<MODULUS>) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl<const MODULUS: InnerType> Neg for Modint<MODULUS> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::raw((MODULUS - self.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> AddAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn add_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self + rhs;
        }
    }

    impl<const MODULUS: InnerType> SubAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn sub_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self - rhs;
        }
    }

    impl<const MODULUS: InnerType> MulAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn mul_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self * rhs;
        }
    }

    impl<const MODULUS: InnerType> DivAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn div_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self / rhs;
        }
    }

    impl<const MODULUS: InnerType, T> Add<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn add(self, rhs: T) -> Self::Output {
            self + Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Sub<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn sub(self, rhs: T) -> Self::Output {
            self - Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Mul<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn mul(self, rhs: T) -> Self::Output {
            self * Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Div<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn div(self, rhs: T) -> Self::Output {
            self / Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> AddAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn add_assign(&mut self, rhs: T) {
            *self += Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> SubAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn sub_assign(&mut self, rhs: T) {
            *self -= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> MulAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self *= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> DivAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn div_assign(&mut self, rhs: T) {
            *self /= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType> Sum<Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, x| acc + x)
        }
    }

    impl<'a, const MODULUS: InnerType> Sum<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, &x| acc + x)
        }
    }

    impl<const MODULUS: InnerType> Product<Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, x| acc * x)
        }
    }

    impl<'a, const MODULUS: InnerType> Product<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, &x| acc * x)
        }
    }

    impl<const MODULUS: InnerType> Modint<MODULUS> {
        /// Returns the modulus.
        pub fn modulus() -> InnerType {
            MODULUS
        }

        /// Returns a `Modint` instance equivalent to `a`.
        pub fn new<T>(a: T) -> Self
        where
            T: Reminder,
        {
            Self {
                rem: a.reminder(MODULUS),
            }
        }

        /// Creates a `Modint` instance from a non-negative integer less than `MODULUS`.
        pub fn raw(a: InnerType) -> Self {
            Self { rem: a }
        }

        /// Set the remainder of `Modint` instance to `a`.
        pub fn set_rem<T>(&mut self, a: T)
        where
            T: Reminder,
        {
            self.rem = a.reminder(MODULUS);
        }

        /// Returns `x` such that `x * q` is equivalent to `p`.
        pub fn frac<T>(p: T, q: T) -> Self
        where
            T: Reminder,
        {
            Self::new(p) / Self::new(q)
        }

        /// Returns the remainder divided by `MODULUS`.
        /// The returned value is a non-negative integer less than `MODULUS`.
        pub fn rem(self) -> InnerType {
            self.rem
        }

        /// Returns the modular multiplicative inverse.
        pub fn inv(self) -> Self {
            Self {
                rem: modinv(self.rem, MODULUS),
            }
        }

        /// Calculates the power of `exp` using the iterative squaring method.
        pub fn pow<T>(self, exp: T) -> Self
        where
            T: ToExponent,
        {
            let mut ret = Self::new(1);
            let mut mul = self;
            let exp = exp.to_exponent();
            let mut t = exp.abs;

            while t != 0 {
                if t & 1 == 1 {
                    ret *= mul;
                }

                mul *= mul;
                t >>= 1;
            }

            if exp.neg {
                ret = ret.inv();
            }

            ret
        }
    }

    pub struct Exponent {
        neg: bool,
        abs: u128,
    }

    pub trait ToExponent {
        fn to_exponent(self) -> Exponent;
    }

    macro_rules! impl_to_exponent_for_unsigned_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: false,
                            abs: self as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_unsigned_int!(usize, u8, u16, u32, u64, u128);

    macro_rules! impl_to_exponent_for_signed_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: self.is_negative(),
                            abs: self.abs() as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_signed_int!(isize, i8, i16, i32, i64, i128);

    #[derive(Debug, Clone)]
    pub struct Factorial<Modint> {
        /// Upper limit of available factorial argument.
        upper_limit: usize,

        /// List of factorials.
        fac: Vec<Modint>,

        /// List of factorial inverses.
        inv_fac: Vec<Modint>,
    }

    impl<const MODULUS: InnerType> Factorial<Modint<MODULUS>> {
        /// Calculates factorial and its inverse for non-negative integers bellow `upper_limit`.
        pub fn new(upper_limit: usize) -> Self {
            let mut fac = vec![Modint::new(1); upper_limit + 1];
            for i in 0..upper_limit {
                fac[i + 1] = fac[i] * (i + 1);
            }

            let mut inv_fac = vec![fac[upper_limit].inv(); upper_limit + 1];
            for i in (0..upper_limit).rev() {
                inv_fac[i] = inv_fac[i + 1] * (i + 1);
            }

            Self {
                upper_limit,
                fac,
                inv_fac,
            }
        }

        /// Returns the factorial `n`.
        pub fn factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.fac[n]
        }

        /// Returns the inverse of the factorial of `n`.
        pub fn inverse_factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.inv_fac[n]
        }

        /// Calculates the number of ways to select and arrange `k` objects from `n` unique objects.
        pub fn permutations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects.
        pub fn combinations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k) * self.inverse_factorial(k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects, allowing for duplicates.
        pub fn combinations_with_repetition(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n == 0 {
                return if k == 0 {
                    Modint::new(1)
                } else {
                    Modint::new(0)
                };
            }

            self.combinations(n + k - 1, k)
        }
    }

    /// The type `Modint` with 1000000007 as the modulus.
    pub type Modint1000000007 = Modint<1000000007>;

    /// The type `Modint` with 998244353 as the modulus.
    pub type Modint998244353 = Modint<998244353>;
}

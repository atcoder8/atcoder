use modint2::Modint998244353;
use proconio::input;

type Mint = Modint998244353;

const MAX: usize = 2 * 10_usize.pow(6) + 2;

fn main() {
    input! {
        t: usize,
        hw: [(usize, usize); t],
    }

    let mut aa = vec![Mint::new(1), Mint::new(1)];
    aa.reserve(MAX + 1);
    for i in 2..=MAX {
        aa.push(aa[i - 2] + aa[i - 1]);
    }

    let mut prod = vec![Mint::new(1)];
    prod.reserve(MAX + 2);
    for (i, &a) in aa.iter().skip(2).step_by(2).enumerate() {
        prod.push(prod[i] * a);
    }

    for (h, w) in hw {
        let (min, max) = (h.min(w), h.max(w));
        let ans = prod[min].pow(2) * aa[2 * min + 1].pow(max - min);
        println!("{}", ans);
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

    /// The type `Modint` with 1000000007 as the modulus.
    pub type Modint1000000007 = Modint<1000000007>;

    /// The type `Modint` with 998244353 as the modulus.
    pub type Modint998244353 = Modint<998244353>;
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

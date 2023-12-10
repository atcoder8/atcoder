use itertools::Itertools;
use lazy_segtree::{Mapping, Monoid};
use modint2::Modint998244353;
use proconio::{input, marker::Usize1};

use crate::lazy_segtree::LazySegtree;

type Mint = Modint998244353;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        lrx: [(Usize1, usize, usize); m],
    }

    let mut segtree =
        LazySegtree::<S, F>::from(aa.iter().map(|&a| S { exp: Mint::new(a) }).collect_vec());
    for (l, r, x) in lrx {
        segtree.apply_range(
            l..r,
            F {
                replace_p: Mint::new(r - l).inv(),
                replaced_exp: Mint::new(x),
            },
        );
    }

    println!("{}", (0..n).map(|i| segtree.get(i).exp).join(" "));
}

#[derive(Debug, Clone, Copy)]
struct S {
    exp: Mint,
}

impl Monoid for S {
    fn id() -> Self {
        Self { exp: Mint::new(0) }
    }

    fn product(&self, _rhs: &Self) -> Self {
        Self { exp: Mint::new(0) }
    }
}

#[derive(Debug, Clone, Copy)]
struct F {
    replace_p: Mint,
    replaced_exp: Mint,
}

impl Monoid for F {
    fn id() -> Self {
        Self {
            replace_p: Mint::new(0),
            replaced_exp: Mint::new(0),
        }
    }

    fn product(&self, rhs: &Self) -> Self {
        let sum_replace_p = self.replace_p + rhs.replace_p - self.replace_p * rhs.replace_p;
        let t = self.replace_p * (Mint::new(1) - rhs.replace_p) * self.replaced_exp
            + rhs.replace_p * rhs.replaced_exp;
        let replaced_exp = if sum_replace_p == Mint::new(0) {
            Mint::new(0)
        } else {
            t / sum_replace_p
        };

        Self {
            replaced_exp,
            replace_p: sum_replace_p,
        }
    }
}

impl Mapping<S> for F {
    fn mapping(&self, s: &S) -> S {
        S {
            exp: (Mint::new(1) - self.replace_p) * s.exp + self.replace_p * self.replaced_exp,
        }
    }
}

pub mod lazy_segtree {
    use std::ops::RangeBounds;

    pub trait Monoid {
        /// The identity element.
        fn id() -> Self;

        /// The binary operation.
        fn product(&self, rhs: &Self) -> Self;
    }

    pub trait Mapping<S>: Monoid
    where
        S: Monoid,
    {
        fn mapping(&self, s: &S) -> S;
    }

    #[derive(Debug, Default, Clone)]
    pub struct LazySegtree<S, F>
    where
        S: Monoid,
        F: Mapping<S>,
    {
        n: usize,
        log: u32,
        size: usize,
        data: Vec<S>,
        lazy: Vec<F>,
    }

    impl<S, F> From<Vec<S>> for LazySegtree<S, F>
    where
        S: Clone + Monoid,
        F: Clone + Mapping<S>,
    {
        fn from(value: Vec<S>) -> Self {
            if value.is_empty() {
                return Self {
                    n: 0,
                    log: 0,
                    size: 0,
                    data: vec![],
                    lazy: vec![],
                };
            }

            let n = value.len();
            let log = usize::BITS - (n - 1).leading_zeros();
            let size = 1 << log;
            let mut data = vec![S::id(); size];
            data.extend(value);
            data.extend(vec![S::id(); size - n]);

            let mut seg = Self {
                n,
                log,
                size,
                data,
                lazy: vec![F::id(); size],
            };

            for i in (1..size).rev() {
                seg.up_product(i);
            }

            seg
        }
    }

    impl<S, F> LazySegtree<S, F>
    where
        S: Monoid,
        F: Mapping<S>,
    {
        pub fn new(n: usize) -> Self
        where
            S: Clone,
            F: Clone,
        {
            if n == 0 {
                return Self {
                    n: 0,
                    log: 0,
                    size: 0,
                    data: vec![],
                    lazy: vec![],
                };
            }

            let log = usize::BITS - (n - 1).leading_zeros();
            let size = 1 << log;

            Self {
                n,
                log,
                size,
                data: vec![S::id(); 2 * size],
                lazy: vec![F::id(); size],
            }
        }

        fn get_bounds<R>(&self, rng: R) -> (usize, usize)
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

            (l, r)
        }

        fn up_product(&mut self, k: usize) {
            assert!(1 <= k && k < self.size);

            self.data[k] = self.data[2 * k].product(&self.data[2 * k + 1]);
        }

        fn down_composite(&mut self, k: usize) {
            assert!(1 <= k && k < self.size);

            let left_child = 2 * k;
            self.data[left_child] = self.lazy[k].mapping(&self.data[left_child]);

            let right_child = left_child + 1;
            self.data[right_child] = self.lazy[k].mapping(&self.data[right_child]);

            if left_child < self.size {
                self.lazy[left_child] = self.lazy[left_child].product(&self.lazy[k]);
                self.lazy[right_child] = self.lazy[right_child].product(&self.lazy[k]);
            }

            self.lazy[k] = F::id();
        }

        fn composite(&mut self, p: usize) {
            assert!(p < self.n);

            let p = p + self.size;

            for i in (1..=self.log).rev() {
                self.down_composite(p >> i);
            }
        }

        pub fn get(&mut self, p: usize) -> &S {
            self.composite(p);

            &self.data[p + self.size]
        }

        pub fn set(&mut self, p: usize, x: S) {
            self.composite(p);

            let p = p + self.size;

            self.data[p] = x;
            for i in 1..=self.log {
                self.up_product(p >> i);
            }
        }

        pub fn apply(&mut self, p: usize, f: F) {
            self.composite(p);

            let p = p + self.size;

            self.data[p] = f.mapping(&self.data[p]);
            for i in 1..=self.log {
                self.up_product(p >> i);
            }
        }

        fn composite_range<R>(&mut self, rng: R)
        where
            R: RangeBounds<usize>,
        {
            let (mut left, mut right) = self.get_bounds(rng);

            assert!(left <= right && right <= self.n);

            left += self.size;
            right += self.size;

            for i in (left.trailing_zeros() + 1..=self.log).rev() {
                self.down_composite(left >> i);
            }

            for i in (right.trailing_zeros() + 1..=self.log).rev() {
                self.down_composite((right - 1) >> i);
            }
        }

        pub fn product_range<R>(&mut self, rng: R) -> S
        where
            R: RangeBounds<usize>,
        {
            let (left, right) = self.get_bounds(rng);

            self.composite_range(left..right);

            let mut left_product = S::id();
            let mut right_product = S::id();

            let mut left = left + self.size;
            let mut right = right + self.size;

            while left < right {
                if left & 1 == 1 {
                    left_product = left_product.product(&self.data[left]);
                    left += 1;
                }

                if right & 1 == 1 {
                    right -= 1;
                    right_product = self.data[right].product(&right_product);
                }

                left >>= 1;
                right >>= 1;
            }

            left_product.product(&right_product)
        }

        pub fn product_all(&self) -> &S {
            &self.data[1]
        }

        pub fn apply_segment(&mut self, k: usize, f: &F) {
            self.data[k] = f.mapping(&self.data[k]);
            if k < self.size {
                self.lazy[k] = self.lazy[k].product(f);
            }
        }

        pub fn apply_range<R>(&mut self, rng: R, f: F)
        where
            R: RangeBounds<usize>,
        {
            let (mut left, mut right) = self.get_bounds(rng);

            self.composite_range(left..right);

            left += self.size;
            right += self.size;

            {
                let mut left = left;
                let mut right = right;

                while left < right {
                    if left & 1 == 1 {
                        self.apply_segment(left, &f);
                        left += 1;
                    }

                    if right & 1 == 1 {
                        right -= 1;
                        self.apply_segment(right, &f);
                    }

                    left >>= 1;
                    right >>= 1;
                }
            }

            for i in left.trailing_zeros() + 1..=self.log {
                self.up_product(left >> i);
            }

            for i in right.trailing_zeros() + 1..=self.log {
                self.up_product((right - 1) >> i);
            }
        }

        pub fn apply_all(&mut self, f: F) {
            self.apply_segment(1, &f);
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

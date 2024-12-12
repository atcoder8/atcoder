use itertools::Itertools;
use modint2::{Factorial, Modint998244353};
use proconio::input;

type Mint = Modint998244353;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    println!("{}", solve(aa));
}

fn solve(aa: Vec<u32>) -> Mint {
    let n = aa.len();

    let factorial = Factorial::<Mint>::new(aa.len());

    let mut double_factorial = vec![Mint::new(1); n + 1];
    for i in (3..=n).step_by(2) {
        double_factorial[i] = i * double_factorial[i - 2];
    }

    let compressed = aa.iter().dedup_with_count().collect_vec();

    if *compressed[0].1 == 0 || compressed.iter().any(|&(cnt, _)| cnt % 2 == 0) {
        return Mint::new(0);
    }

    let mut ans = Mint::new(1);
    let mut sum_num_operations = 0_usize;
    for &(cnt, _) in &compressed {
        if cnt == 1 {
            continue;
        }

        let num_operations = (cnt - 1) / 2;
        ans *= factorial.combinations_with_repetition(sum_num_operations + 1, num_operations)
            * double_factorial[cnt - 2];
        sum_num_operations += num_operations;
    }

    ans
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{rngs::ThreadRng, Rng};

    use super::*;

    /// Input data type.
    type Input = Vec<u32>;

    /// Output data type.
    type Output = Mint;

    /// Performs the specified number of tests.
    #[test]
    fn test() {
        /// This value specifies the number of tests.
        const NUMBER_OF_TESTS: usize = 1000;

        let mut rng = rand::thread_rng();

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator(&mut rng);
            let expected_output = expected(input.clone());
            let actual_output = actual(input.clone());

            // If an unexpected answer is returned, panic is triggered.
            assert_eq!(
                expected_output, actual_output,
                "
Unexpected answer was returned in test case #{}.

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
                test_case_index, input, expected_output, actual_output
            );
        }
    }

    /// Generates a test case.
    pub fn generator(rng: &mut ThreadRng) -> Input {
        let n = rng.gen_range(1..=5);
        let aa = (0..n).map(|_| rng.gen_range(0_u32..2)).collect_vec();

        aa
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let aa = input;
        let n = aa.len();

        let mut num_combs = Mint::new(0);
        let init_seq = (1..=n).map(|i| (i % 2) as u32).collect_vec();
        let mut stack = vec![init_seq];
        while let Some(seq) = stack.pop() {
            if seq == aa {
                num_combs += 1;
                continue;
            }

            for left in 0..n {
                for right in left + 2..n {
                    if seq[left] == seq[right]
                        && seq[left + 1..right].iter().all(|&val| val != seq[left])
                    {
                        let mut next_seq = seq.clone();
                        next_seq[left + 1..right].fill(seq[left]);
                        stack.push(next_seq);
                    }
                }
            }
        }

        num_combs
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let aa = input;

        solve(aa)
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

    macro_rules! impl_ops_for_integer {
        ($($integer:ty), *) => {
            $(
                impl<const MODULUS: InnerType> Add<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn add(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) + rhs
                    }
                }

                impl<const MODULUS: InnerType> Sub<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn sub(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) - rhs
                    }
                }

                impl<const MODULUS: InnerType> Mul<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn mul(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) * rhs
                    }
                }

                impl<const MODULUS: InnerType> Div<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn div(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) / rhs
                    }
                }
            )*
        };
    }

    impl_ops_for_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

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

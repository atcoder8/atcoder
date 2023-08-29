use itertools::Itertools;
use modint2::Modint998244353;
use num_integer::Integer;
use proconio::input;

type Mint = Modint998244353;

const APPEND_BITS: [usize; 4] = [0b000, 0b011, 0b101, 0b110];

fn main() {
    input! {
        (n, aa): (usize, [usize; 3]),
    }

    let mut dp = vec![vec![vec![vec![Mint::new(0); aa[2]]; aa[1]]; aa[0]]; 8];
    dp[0b111][0][0][0] = Mint::new(1);
    for digit_idx in (0..60).rev() {
        let mut next_dp = vec![vec![vec![vec![Mint::new(0); aa[2]]; aa[1]]; aa[0]]; 8];

        let d = (n >> digit_idx) & 1;

        for (ss, bb) in (0..8).cartesian_product(APPEND_BITS) {
            let next_ss = match calc_next_ss(ss, d, bb) {
                Some(next_ss) => next_ss,
                None => continue,
            };

            for r0 in 0..aa[0] {
                let next_r0 = (2 * r0 + (bb & 1)) % aa[0];

                for r1 in 0..aa[1] {
                    let next_r1 = (2 * r1 + ((bb >> 1) & 1)) % aa[1];

                    for r2 in 0..aa[2] {
                        let next_r2 = (2 * r2 + ((bb >> 2) & 1)) % aa[2];

                        next_dp[next_ss][next_r0][next_r1][next_r2] += dp[ss][r0][r1][r2];
                    }
                }
            }
        }

        dp = next_dp;
    }

    let mut ans = Mint::new(0);
    for ss in 0..8 {
        ans += dp[ss][0][0][0];
    }
    for i in 0..3 {
        ans -= n / aa[i].lcm(&aa[(i + 1) % 3]);
    }
    ans -= 1;

    println!("{}", ans);
}

fn calc_next_ss(ss: usize, d: usize, bb: usize) -> Option<usize> {
    let mut next_ss = 0;
    for i in 0..3 {
        let (s, b) = ((ss >> i) & 1, (bb >> i) & 1);
        let next_s = s * 1_usize.wrapping_sub(d.wrapping_sub(b));

        if next_s == 2 {
            return None;
        }

        next_ss += next_s << i;
    }

    Some(next_ss)
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

    #[cfg(test)]
    mod tests {
        use super::*;

        type Mint = Modint1000000007;

        #[test]
        fn test_reminder_998244353() {
            assert_eq!(
                Modint998244353::new(2 * 998244353 + 3),
                Modint998244353::new(3)
            );
        }

        #[test]
        fn test_reminder_1000000007() {
            assert_eq!(
                Modint1000000007::new(2 * 1000000007 + 3),
                Modint1000000007::new(3)
            );
        }

        #[test]
        fn test_zero() {
            assert_eq!(Mint::new(0).rem(), 0);
        }

        #[test]
        fn test_positive() {
            assert_eq!(Mint::new(10).rem(), 10);
        }

        #[test]
        fn test_negative() {
            assert_eq!(Mint::new(-3), Mint::new(Mint::modulus() - 3));
        }

        #[test]
        fn test_inverse() {
            assert_eq!(Mint::new(5) * Mint::new(5).inv(), Mint::new(1));
        }

        #[test]
        fn test_add() {
            assert_eq!(
                Mint::new(Mint::modulus() - 10) + Mint::new(50),
                Mint::new(40)
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(Mint::new(10) - 20, Mint::new(-10));
        }

        #[test]
        fn test_mul() {
            assert_eq!(
                Mint::new(123456789) * Mint::new(987654321),
                Mint::new(259106859)
            )
        }

        #[test]
        fn test_div_35_divide_by_5() {
            assert_eq!(Mint::new(35) / Mint::new(5), Mint::new(7));
        }

        #[test]
        fn test_div_37_divide_by_5() {
            assert_eq!(Mint::new(37) / 5, Mint::new(800000013));
        }

        #[test]
        fn test_neg() {
            assert_eq!(Mint::new(3).neg(), Mint::new(-3));
        }

        #[test]
        fn test_add_assign() {
            let mut lhs = Mint::new(3);
            let rhs = Mint::new(5);

            lhs += rhs;

            assert_eq!(lhs, Mint::new(8));
        }

        #[test]
        fn test_sub_assign() {
            let mut lhs = Mint::new(10);
            let rhs = Mint::new(3);

            lhs -= rhs;

            assert_eq!(lhs, Mint::new(7));
        }

        #[test]
        fn test_mul_assign() {
            let mut lhs = Mint::new(10_u32.pow(9));
            let rhs = Mint::new(10);

            lhs *= rhs;

            assert_eq!(lhs, Mint::new(999999937));
        }

        #[test]
        fn test_div_assign() {
            let mut lhs = Mint::new(1);
            let rhs = Mint::new(10);

            lhs /= rhs;

            assert_eq!(lhs, Mint::new(700000005));
        }

        #[test]
        fn test_power_0_of_100() {
            assert_eq!(Mint::new(100).pow(0).rem(), 1);
        }

        #[test]
        fn test_power_100_of_2() {
            assert_eq!(Mint::new(2).pow(100).rem(), 976371285);
        }

        #[test]
        fn test_power_neg100_of_3() {
            assert_eq!(Mint::new(3).pow(-100).rem(), 35174754);
        }

        #[test]
        fn test_power_quintillion_of_123456789() {
            assert_eq!(Mint::new(123456789).pow(10_u64.pow(18)).rem(), 228100152);
        }

        #[test]
        fn test_power_large() {
            assert_eq!(Mint::new(123456789).pow(10_u128.pow(38)).rem(), 781301325);
        }

        #[test]
        #[should_panic]
        fn test_inverse_of_zero() {
            Mint::new(0).inv();
        }

        #[test]
        #[should_panic]
        fn test_not_coprime() {
            Modint::<21>::new(14).inv();
        }

        #[test]
        fn test_into_iter_sum() {
            let seq = vec![
                Mint::new(314159265),
                Mint::new(358979323),
                Mint::new(846264338),
            ];

            assert_eq!(seq.into_iter().sum::<Mint>(), Mint::new(519402919));
        }

        #[test]
        fn test_into_iter_sum_empty() {
            let seq: Vec<Mint> = vec![];

            assert_eq!(seq.into_iter().sum::<Mint>(), Mint::new(0));
        }

        #[test]
        fn test_iter_sum() {
            let seq = vec![
                Mint::new(314159265),
                Mint::new(358979323),
                Mint::new(846264338),
            ];

            assert_eq!(seq.iter().sum::<Mint>(), Mint::new(519402919));
        }

        #[test]
        fn test_iter_sum_empty() {
            let seq: Vec<Mint> = vec![];

            assert_eq!(seq.iter().sum::<Mint>(), Mint::new(0));
        }

        #[test]
        fn test_into_iter_product() {
            let seq = vec![
                Mint::new(314159265),
                Mint::new(358979323),
                Mint::new(846264338),
            ];

            assert_eq!(seq.into_iter().product::<Mint>(), Mint::new(264776062));
        }

        #[test]
        fn test_into_iter_product_empty() {
            let seq: Vec<Mint> = vec![];

            assert_eq!(seq.into_iter().product::<Mint>(), Mint::new(1));
        }

        #[test]
        fn test_iter_product() {
            let seq = vec![
                Mint::new(314159265),
                Mint::new(358979323),
                Mint::new(846264338),
            ];

            assert_eq!(seq.iter().product::<Mint>(), Mint::new(264776062));
        }

        #[test]
        fn test_iter_product_empty() {
            let seq: Vec<Mint> = vec![];

            assert_eq!(seq.iter().product::<Mint>(), Mint::new(1));
        }
    }
}

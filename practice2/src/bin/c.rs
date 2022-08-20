use modint::ModInt1000000007 as mint;
use num_traits::Pow;

use crate::factorial::Factorial;

fn main() {
    let (n, a, b): (usize, usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut fac = Factorial::<mint>::new();

    let ans: mint = mint::from(2).pow(n) - 1 - fac.combination(n, a) - fac.combination(n, b);
    println!("{}", ans.get_val());
}

pub mod factorial {
    use std::cmp;
    use std::ops::{Div, Mul};

    pub struct Factorial<T> {
        fac: Vec<T>,
    }

    impl<T> Factorial<T>
    where
        T: Copy + From<usize> + Mul<Output = T> + Div<Output = T>,
    {
        pub fn new() -> Self {
            Self {
                fac: vec![T::from(1)],
            }
        }

        // Returns the factorial of n.
        pub fn factorial(&mut self, n: usize) -> T {
            if self.fac.len() < n + 1 {
                for i in (self.fac.len() - 1)..n {
                    self.fac.push(self.fac[i] * (i + 1).into());
                }
            }
            self.fac[n]
        }

        // Returns the number of choices when selecting k from n and arranging them in a row.
        pub fn permutation(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / self.factorial(cmp::min(k, n - k))
            }
        }

        // Returns the number of choices to select k from n.
        pub fn combination(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / (self.factorial(k) * self.factorial(n - k))
            }
        }
    }
}

pub mod modint {
    use num::pow::Pow;
    use std::convert::From;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    pub fn modinv(a: u32, m: u32) -> u32 {
        assert!(m >= 2);

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
            "The inverse does not exist. gcd(a, m) = {}",
            a.abs()
        );

        s %= m as i64;
        if s < 0 {
            s += m as i64;
        }

        s as u32
    }

    pub trait RemEuclidU32 {
        fn rem_euclid_u32(self, modulus: u32) -> u32;
    }

    // 32bit以下の符号付き整数型に対してrem_euclid_u32を実装するマクロ
    macro_rules! impl_rem_euclid_u32_for_small_signed {
        ($($small_signed_type:tt),*) => {
            $(
                impl RemEuclidU32 for $small_signed_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        let ret = (self as i32) % (modulus as i32);
                        if ret >= 0 {
                            ret as u32
                        } else {
                            (ret + modulus as i32) as u32
                        }
                    }
                }
            )*
        };
    }

    // 64bitの符号付き整数型(isizeを含む)に対してrem_euclid_u32を実装するマクロ
    macro_rules! impl_rem_euclid_u32_for_large_signed {
        ($($large_signed_type:tt),*) => {
            $(
                impl RemEuclidU32 for $large_signed_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        let ret = (self as i64) % (modulus as i64);
                        if ret >= 0 {
                            ret as u32
                        } else {
                            (ret + modulus as i64) as u32
                        }
                    }
                }
            )*
        };
    }

    // 32bit以上の符号無し整数型に対してrem_euclid_u32を実装するマクロ
    macro_rules! impl_rem_euclid_u32_for_small_unsigned {
        ($($small_unsigned_type:tt),*) => {
            $(
                impl RemEuclidU32 for $small_unsigned_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        self as u32 % modulus
                    }
                }
            )*
        };
    }

    // 64bit以上の符号無し整数型(usizeを含む)に対してrem_euclid_u32を実装するマクロ
    macro_rules! impl_rem_euclid_u32_for_large_unsigned {
        ($($large_unsigned_type:tt),*) => {
            $(
                impl RemEuclidU32 for $large_unsigned_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        (self % modulus as $large_unsigned_type) as u32
                    }
                }
            )*
        };
    }

    // 32bit以下の符号付き整数型に対してrem_euclid_u32を実装
    impl_rem_euclid_u32_for_small_signed!(i8, i16, i32);

    // 64bitの符号付き整数型(isizeを含む)に対してrem_euclid_u32を実装
    impl_rem_euclid_u32_for_large_signed!(i64, isize);

    // 32bit以上の符号無し整数型に対してrem_euclid_u32を実装
    impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);

    // 64bit以上の符号無し整数型(usizeを含む)に対してrem_euclid_u32を実装
    impl_rem_euclid_u32_for_large_unsigned!(u64, u128, usize);

    // 128bitの符号付き整数型に対してrem_euclid_u32を実装
    impl RemEuclidU32 for i128 {
        fn rem_euclid_u32(self, modulus: u32) -> u32 {
            let ret = self % (modulus as i128);
            if ret >= 0 {
                ret as u32
            } else {
                (ret + modulus as i128) as u32
            }
        }
    }

    #[macro_export]
    macro_rules! generate_modint {
        // 型名と法を指定してModIntを生成
        ($modint_type:tt, $mod:literal) => {
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct $modint_type {
                val: u32,
            }

            impl $modint_type {
                const MOD: u32 = $mod;

                // 法(modulus)を返します。
                pub fn get_mod() -> u32 {
                    Self::MOD
                }
            }

            impl $modint_type {
                pub fn raw(val: u32) -> Self {
                    Self { val }
                }

                pub fn get_val(&self) -> u32 {
                    self.val
                }

                pub fn inv(&self) -> Self {
                    Self::from(modinv(self.val, $mod))
                }
            }

            impl<T: RemEuclidU32> From<T> for $modint_type {
                fn from(val: T) -> Self {
                    Self {
                        val: val.rem_euclid_u32($mod),
                    }
                }
            }

            impl Add for $modint_type {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self::from(self.val + rhs.val)
                }
            }

            impl Sub for $modint_type {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    Self::from(self.val + $mod - rhs.val)
                }
            }

            impl Mul for $modint_type {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    Self::from(self.val as u64 * rhs.val as u64)
                }
            }

            impl Div for $modint_type {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    self * rhs.inv()
                }
            }

            impl AddAssign for $modint_type {
                fn add_assign(&mut self, other: Self) {
                    *self = *self + other;
                }
            }

            impl SubAssign for $modint_type {
                fn sub_assign(&mut self, other: Self) {
                    *self = *self - other;
                }
            }

            impl MulAssign for $modint_type {
                fn mul_assign(&mut self, other: Self) {
                    *self = *self * other;
                }
            }

            impl DivAssign for $modint_type {
                fn div_assign(&mut self, other: Self) {
                    *self = *self / other;
                }
            }

            impl Neg for $modint_type {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self::from(Self::MOD - self.val)
                }
            }

            // 各整数型に対して$modint_typeとの二項演算を定義
            impl_ops!(
                $modint_type,
                i8,
                i16,
                i32,
                i64,
                i128,
                isize,
                u8,
                u16,
                u32,
                u64,
                u128,
                usize
            );

            // 符号無し整数型に対してModIntの冪乗を定義
            impl_power_for_unsigned!($modint_type, u8, u16, u32, u64, u128, usize);

            // 32bit以下の符号付き整数型に対してModIntの冪乗を定義
            impl_power_for_small_signed!($modint_type, i8, i16, i32);

            // 64bitの符号付き整数型(isizeを含む)に対してModIntの冪乗を定義
            impl_power_for_large_signed!($modint_type, i64, isize);

            // 128bitの符号付き整数型に対してModIntの冪乗を定義するマクロ
            impl Pow<i128> for $modint_type {
                type Output = Self;

                fn pow(self, n: i128) -> Self::Output {
                    if n >= 0 {
                        self.pow(n as u128)
                    } else {
                        self.pow(-n as u128).inv()
                    }
                }
            }
        };
    }

    // 符号無し整数型に対してModIntの冪乗を定義するマクロ
    macro_rules! impl_power_for_unsigned {
        ($modint_type:tt, $($unsigned_type:tt),*) => {
            $(
                impl Pow<$unsigned_type> for $modint_type {
                    type Output = Self;

                    fn pow(self, mut n: $unsigned_type) -> Self::Output {
                        let mut ret = Self::from(1);
                        let mut mul = self;
                        while n != 0 {
                            if n & 1 == 1 {
                                ret *= mul;
                            }
                            mul *= mul;
                            n >>= 1;
                        }
                        ret
                    }
                }
            )*
        };
    }

    // 32bit以下の符号付き整数型に対してModIntの冪乗を定義するマクロ
    macro_rules! impl_power_for_small_signed {
        ($modint_type:tt, $($small_signed_type:tt),*) => {
            $(
                impl Pow<$small_signed_type> for $modint_type {
                    type Output = Self;

                    fn pow(self, n: $small_signed_type) -> Self::Output {
                        if n >= 0 {
                            self.pow(n as u32)
                        } else {
                            self.pow(-n as u32).inv()
                        }
                    }
                }
            )*
        };
    }

    // 64bitの符号付き整数型(isizeを含む)に対してModIntの冪乗を定義するマクロ
    macro_rules! impl_power_for_large_signed {
        ($modint_type:tt, $($large_signed_type:tt),*) => {
            $(
                impl Pow<$large_signed_type> for $modint_type {
                    type Output = Self;

                    fn pow(self, n: $large_signed_type) -> Self::Output {
                        if n >= 0 {
                            self.pow(n as u64)
                        } else {
                            self.pow(-n as u64).inv()
                        }
                    }
                }
            )*
        };
    }

    // 各整数型に対して$modint_typeとの二項演算をオーバーロードするマクロ
    macro_rules! impl_ops {
        ($modint_type:tt, $($other_type:tt),*) => {
            $(
                impl Add<$other_type> for $modint_type {
                    type Output = Self;

                    fn add(self, rhs: $other_type) -> Self::Output {
                        self + Self::from(rhs)
                    }
                }

                impl Add<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn add(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::from(self) + rhs
                    }
                }

                impl Sub<$other_type> for $modint_type {
                    type Output = Self;

                    fn sub(self, rhs: $other_type) -> Self::Output {
                        self - Self::from(rhs)
                    }
                }

                impl Sub<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn sub(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::from(self) - rhs
                    }
                }

                impl Mul<$other_type> for $modint_type {
                    type Output = Self;

                    fn mul(self, rhs: $other_type) -> Self::Output {
                        self * Self::from(rhs)
                    }
                }

                impl Mul<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn mul(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::from(self) * rhs
                    }
                }

                impl Div<$other_type> for $modint_type {
                    type Output = Self;

                    fn div(self, rhs: $other_type) -> Self::Output {
                        self / Self::from(rhs)
                    }
                }

                impl Div<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn div(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::from(self) / rhs
                    }
                }

                impl AddAssign<$other_type> for $modint_type {
                    fn add_assign(&mut self, other: $other_type) {
                        *self = *self + Self::from(other);
                    }
                }

                impl SubAssign<$other_type> for $modint_type {
                    fn sub_assign(&mut self, other: $other_type) {
                        *self = *self - Self::from(other);
                    }
                }

                impl MulAssign<$other_type> for $modint_type {
                    fn mul_assign(&mut self, other: $other_type) {
                        *self = *self * Self::from(other);
                    }
                }

                impl DivAssign<$other_type> for $modint_type {
                    fn div_assign(&mut self, other: $other_type) {
                        *self = *self / Self::from(other);
                    }
                }
            )*
        };
    }

    // 998244353を法とするModIntを定義
    generate_modint!(ModInt998244353, 998244353);

    // 1000000007を法とするModIntを定義
    generate_modint!(ModInt1000000007, 1000000007);
}

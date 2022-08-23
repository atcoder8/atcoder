use std::collections::HashSet;

use atcoder8_library::modint::ModInt998244353;

type Mint = ModInt998244353;

fn main() {
    let (n, m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let (a, b, c, d, e, f) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let mut xy = Vec::new();
    for _ in 0..m {
        xy.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<i64>().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        });
    }
    let mut barrier = HashSet::new();
    for &t in xy.iter() {
        barrier.insert(t);
    }

    let mut dp = vec![vec![vec![Mint::new(0); n + 1]; n + 1]; n + 1];
    dp[0][0][0] = Mint::new(1);

    for i in 0..n {
        for j in 0..(n - i) {
            for k in 0..(n - i - j) {
                let coord = (
                    a * i as i64 + c * j as i64 + e * k as i64,
                    b * i as i64 + d * j as i64 + f * k as i64,
                );
                if barrier.contains(&coord) {
                    continue;
                }

                let t = dp[i][j][k];
                dp[i + 1][j][k] += t;
                dp[i][j + 1][k] += t;
                dp[i][j][k + 1] += t;
            }
        }
    }

    let mut ans = Mint::new(0);
    for i in 0..(n + 1) {
        for j in 0..(n + 1 - i) {
            let k = n - i - j;

            let coord = (
                a * i as i64 + c * j as i64 + e * k as i64,
                b * i as i64 + d * j as i64 + f * k as i64,
            );
            if barrier.contains(&coord) {
                continue;
            }

            ans += dp[i][j][k];
        }
    }

    println!("{}", ans.get_val());
}

pub mod atcoder8_library {
    pub mod modint {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign,
        };

        pub trait RemEuclidU32 {
            fn rem_euclid_u32(self, modulus: u32) -> u32;
        }

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

        pub trait Pow<T: Copy + ShrAssign> {
            fn pow(self, n: T) -> Self;
        }

        #[macro_export]
        macro_rules! generate_modint {
            // 型名と法を指定してModIntを生成
            ($modint_type:tt, $modulus:literal) => {
                #[derive(Clone, Copy, PartialEq, Eq)]
                pub struct $modint_type {
                    val: u32,
                }

                impl $modint_type {
                    const MOD: u32 = $modulus;
                }

                impl $modint_type {
                    pub fn new<T: RemEuclidU32>(val: T) -> Self {
                        Self {
                            val: val.rem_euclid_u32($modulus),
                        }
                    }

                    pub fn raw(val: u32) -> Self {
                        Self { val }
                    }

                    pub fn get_val(&self) -> u32 {
                        self.val
                    }

                    pub fn inv(&self) -> Self {
                        Self::new(modinv(self.val, $modulus))
                    }
                }

                impl<T: RemEuclidU32> From<T> for $modint_type {
                    fn from(val: T) -> Self {
                        Self::new(val)
                    }
                }

                impl Add for $modint_type {
                    type Output = Self;

                    fn add(self, rhs: Self) -> Self::Output {
                        Self::new(self.val + rhs.val)
                    }
                }

                impl Sub for $modint_type {
                    type Output = Self;

                    fn sub(self, rhs: Self) -> Self::Output {
                        Self::new(self.val + $modulus - rhs.val)
                    }
                }

                impl Mul for $modint_type {
                    type Output = Self;

                    fn mul(self, rhs: Self) -> Self::Output {
                        Self::new(self.val as u64 * rhs.val as u64)
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
                        Self::new(Self::MOD - self.val)
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
                    fn pow(self, n: i128) -> Self {
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
                        fn pow(self, mut n: $unsigned_type) -> Self {
                            let mut ret = Self::new(1);
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
                        fn pow(self, n: $small_signed_type) -> Self {
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
                        fn pow(self, n: $large_signed_type) -> Self {
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
                            self + Self::new(rhs)
                        }
                    }

                    impl Add<$modint_type> for $other_type {
                        type Output = $modint_type;

                        fn add(self, rhs: $modint_type) -> Self::Output {
                            $modint_type::new(self) + rhs
                        }
                    }

                    impl Sub<$other_type> for $modint_type {
                        type Output = Self;

                        fn sub(self, rhs: $other_type) -> Self::Output {
                            self - Self::new(rhs)
                        }
                    }

                    impl Sub<$modint_type> for $other_type {
                        type Output = $modint_type;

                        fn sub(self, rhs: $modint_type) -> Self::Output {
                            $modint_type::new(self) - rhs
                        }
                    }

                    impl Mul<$other_type> for $modint_type {
                        type Output = Self;

                        fn mul(self, rhs: $other_type) -> Self::Output {
                            self * Self::new(rhs)
                        }
                    }

                    impl Mul<$modint_type> for $other_type {
                        type Output = $modint_type;

                        fn mul(self, rhs: $modint_type) -> Self::Output {
                            $modint_type::new(self) * rhs
                        }
                    }

                    impl Div<$other_type> for $modint_type {
                        type Output = Self;

                        fn div(self, rhs: $other_type) -> Self::Output {
                            self / Self::new(rhs)
                        }
                    }

                    impl Div<$modint_type> for $other_type {
                        type Output = $modint_type;

                        fn div(self, rhs: $modint_type) -> Self::Output {
                            $modint_type::new(self) / rhs
                        }
                    }

                    impl AddAssign<$other_type> for $modint_type {
                        fn add_assign(&mut self, other: $other_type) {
                            *self = *self + Self::new(other);
                        }
                    }

                    impl SubAssign<$other_type> for $modint_type {
                        fn sub_assign(&mut self, other: $other_type) {
                            *self = *self - Self::new(other);
                        }
                    }

                    impl MulAssign<$other_type> for $modint_type {
                        fn mul_assign(&mut self, other: $other_type) {
                            *self = *self * Self::new(other);
                        }
                    }

                    impl DivAssign<$other_type> for $modint_type {
                        fn div_assign(&mut self, other: $other_type) {
                            *self = *self / Self::new(other);
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
}

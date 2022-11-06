use atcoder8_library::modint::Modint998244353;
use proconio::input;

use crate::atcoder8_library::fenwick_tree::FenwickTree;

type Mint = Modint998244353;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = Mint::new(0);
    let mut ft_for_cnt: FenwickTree<usize> = FenwickTree::new(200_001);
    let mut ft_for_card: FenwickTree<usize> = FenwickTree::new(200_001);

    for (i, &a) in aa.iter().enumerate() {
        ans *= i.pow(2);
        ans += 2 * ft_for_card.sum((a + 1)..);
        ans += a * (2 * ft_for_cnt.sum(..=a) + 1);
        ans /= (i + 1).pow(2);

        ft_for_cnt.add(a, 1);
        ft_for_card.add(a, a);

        println!("{}", ans.val());
    }
}

pub mod atcoder8_library {
    pub mod fenwick_tree {
        //! Processes the following query in `O(log(n))` time
        //! for a sequence of numbers with `n` elements:
        //! * Update one element
        //! * Calculate the sum of the elements of a range
        //! * Gets the elements of a number sequence.

        use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};

        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..), 11);
        /// ```
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct FenwickTree<T>(Vec<T>)
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T>;

        impl<T> From<Vec<T>> for FenwickTree<T>
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T>,
        {
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 9);
            /// ```
            fn from(t: Vec<T>) -> Self {
                let mut ft = FenwickTree::new(t.len());
                for (i, x) in t.into_iter().enumerate() {
                    ft.add(i, x);
                }
                ft
            }
        }

        impl<T> FenwickTree<T>
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T>,
        {
            /// Constructs a `FenwickTree<T>` with `n` elements.
            ///
            /// Each element is initialized with `T::default()`.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::<i32>::new(5);
            /// assert_eq!(ft.sum(..), 0);
            /// ```
            pub fn new(n: usize) -> Self {
                FenwickTree(vec![T::default(); n])
            }

            /// Add `x` to the `p`-th element.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 9);
            ///
            /// ft.add(3, 100);
            /// assert_eq!(ft.sum(2..6), 109);
            /// ```
            pub fn add(&mut self, p: usize, x: T) {
                let FenwickTree(data) = self;
                let n = data.len();

                assert!(p < n);

                let mut p = p + 1;
                while p <= n {
                    data[p - 1] += x.clone();
                    p += p & p.overflowing_neg().0;
                }
            }

            /// Sets `x` to the `p`-th element.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 9);
            ///
            /// ft.set(3, 100);
            /// assert_eq!(ft.sum(2..6), 108);
            /// ```
            pub fn set(&mut self, p: usize, x: T) {
                let FenwickTree(data) = self;
                let n = data.len();

                assert!(p < n);

                let t = x - self.get(p);
                self.add(p, t);
            }

            /// Compute the sum of the range [0, r).
            fn inner_sum(&self, r: usize) -> T {
                let mut r = r;
                let mut s = T::default();
                while r > 0 {
                    s += self.0[r - 1].clone();
                    r -= r & r.overflowing_neg().0;
                }
                return s;
            }

            /// Calculate the total of the range.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(..), 13);
            /// assert_eq!(ft.sum(2..), 11);
            /// assert_eq!(ft.sum(..6), 11);
            /// assert_eq!(ft.sum(2..6), 9);
            /// assert_eq!(ft.sum(6..2), 0);
            /// ```
            pub fn sum<R>(&self, rng: R) -> T
            where
                R: RangeBounds<usize>,
            {
                let n = self.0.len();

                let l = match rng.start_bound() {
                    std::ops::Bound::Included(&start_bound) => start_bound,
                    std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                    std::ops::Bound::Unbounded => 0,
                };

                let r = match rng.end_bound() {
                    std::ops::Bound::Included(&end_bound) => end_bound + 1,
                    std::ops::Bound::Excluded(&end_bound) => end_bound,
                    std::ops::Bound::Unbounded => n,
                };

                assert!(l <= n && r <= n);

                if l >= r {
                    T::default()
                } else {
                    self.inner_sum(r) - self.inner_sum(l)
                }
            }

            /// Returns the value of an element in a sequence of numbers.
            /// Calculate the total of the range.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::from(vec![3, -1, 4, -1, 5]);
            /// assert_eq!(ft.get(2), 4);
            /// ```
            pub fn get(&self, p: usize) -> T {
                assert!(p < self.0.len());

                self.sum(p..=p)
            }
        }

        impl<T> FenwickTree<T>
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T> + SubAssign<T>,
        {
            /// Subtract `x` from the `p`-th element.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let mut ft = FenwickTree::<u32>::from(vec![3, 1, 4, 1, 5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 19);
            ///
            /// ft.sub(3, 1);
            /// assert_eq!(ft.sum(2..6), 18);
            /// ```
            pub fn sub(&mut self, p: usize, x: T) {
                let FenwickTree(data) = self;
                let n = data.len();

                assert!(p < n);

                let mut p = p + 1;
                while p <= n {
                    data[p - 1] -= x.clone();
                    p += p & p.overflowing_neg().0;
                }
            }
        }
    }

    pub mod modint {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign,
        };

        pub trait RemEuclidU32 {
            fn rem_euclid_u32(self, modulus: u32) -> u32;
        }

        /// Calculate the modular multiplicative inverse of `a` with `m` as modulus.
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

        /// This macro implements rem_euclid_u32 for signed integer types of 32 bits or less.
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

        /// This macro implements rem_euclid_u32 for 64-bit signed integer types (including isize).
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

        /// This macro implements rem_euclid_u32 for unsigned integer types greater than 32 bits.
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

        /// This macro implements rem_euclid_u32 for 64-bit and larger unsigned integer types (including usize).
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

        // Implement rem_euclid_u32 for signed integer types of 32 bits or less.
        impl_rem_euclid_u32_for_small_signed!(i8, i16, i32);

        // Implement rem_euclid_u32 for 64-bit signed integer types (including isize).
        impl_rem_euclid_u32_for_large_signed!(i64, isize);

        // Implement rem_euclid_u32 for unsigned integer types of 32 bits or more.
        impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);

        // Implement rem_euclid_u32 for unsigned integer types (including usize) of 64 bits or more.
        impl_rem_euclid_u32_for_large_unsigned!(u64, u128, usize);

        // Implement rem_euclid_u32 for i128.
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

        /// Macro to overload binary operation with `$modint_type` for each integer type
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

        /// This macro defines powers of Modint for unsigned integer types.
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

        /// This macro defines powers of Modint for signed integer types of 32 bits or less.
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

        /// This macro defines the power of Modint for 64-bit signed integer types (including isize).
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

        /// This macro generates Modint by specifying the type name and modulus.
        #[macro_export]
        macro_rules! generate_modint {
            ($modint_type:tt, $modulus:literal) => {
                #[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
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

                    pub fn val(&self) -> u32 {
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

                    #[allow(clippy::suspicious_arithmetic_impl)]
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

                // Define a binary operation between each integer type and $modint_type.
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

                // Define powers of Modint for unsigned integer types.
                impl_power_for_unsigned!($modint_type, u8, u16, u32, u64, u128, usize);

                // Define powers of Modint for signed integer types of 32 bits or less.
                impl_power_for_small_signed!($modint_type, i8, i16, i32);

                // Define Modint powers for 64-bit signed integer types (including isize).
                impl_power_for_large_signed!($modint_type, i64, isize);

                // Define the power of Modint for 128-bit signed integer types.
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

        // Define Modint with 998244353 as modulus
        generate_modint!(Modint998244353, 998244353);

        // Define Modint with 1000000007 as modulus
        generate_modint!(Modint1000000007, 1000000007);
    }
}

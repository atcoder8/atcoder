use atcoder8_library::modint::Modint998244353;
use num::{One, Zero};
use proconio::input;

use crate::atcoder8_library::matrix::{MatPow, Matrix, Vector};

type Mint = Modint998244353;

fn main() {
    input! {
        (n, p): (usize, usize)
    }

    let critical_prob = Mint::frac(p, 100);
    let normal_prob = Mint::new(1) - critical_prob;

    let mat: Matrix<Mint> = vec![
        vec![normal_prob, critical_prob, Mint::one()],
        vec![Mint::one(), Mint::zero(), Mint::zero()],
        vec![Mint::zero(), Mint::zero(), Mint::one()],
    ]
    .into();
    let powered_mat = mat.mat_pow(n - 1);

    let vec: Vector<Mint> = vec![Mint::one(), Mint::zero(), Mint::one()].into();

    let applied_vec = powered_mat.apply_to(&vec);

    let ans = *applied_vec.get(0);
    println!("{}", ans.val());
}

pub mod atcoder8_library {
    pub mod modint {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign,
        };

        use num::{One, Zero};

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

                    pub fn frac<T: RemEuclidU32>(numer: T, denom: T) -> Self {
                        Self::new(numer) / Self::new(denom)
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

                impl Zero for $modint_type {
                    fn zero() -> Self {
                        Self::new(0)
                    }

                    fn is_zero(&self) -> bool {
                        self.val == 0
                    }
                }

                impl One for $modint_type {
                    fn one() -> Self {
                        Self::new(1)
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

    pub mod matrix {
        use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

        use num::{One, Zero};

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Matrix<T>
        where
            T: Clone,
        {
            shape: (usize, usize),
            flattened: Vec<T>,
        }

        pub trait MatMul<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn mat_mul(&self, rhs: &Self) -> Self;

            fn mat_mul_assign(&mut self, rhs: &Self);
        }

        pub trait Transpose<T>
        where
            T: Clone,
        {
            fn transposed(&self) -> Self;
        }

        pub trait MatPow<T>
        where
            T: Clone + Zero + One + MulAssign<T>,
        {
            fn mat_pow(&self, exp: usize) -> Self;
        }

        impl<T> MatMul<T> for Matrix<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn mat_mul(&self, rhs: &Self) -> Self {
                let (h1, w1) = self.shape;
                let (h2, w2) = rhs.shape;

                assert_eq!(w1, h2);

                let calc_elem = |coord: (usize, usize)| {
                    let (i, j) = coord;

                    let mut elem = self.flattened[self.coord_to_idx((i, 0))].clone()
                        * rhs.flattened[rhs.coord_to_idx((0, j))].clone();
                    for k in 1..w1 {
                        elem = elem
                            + self.flattened[self.coord_to_idx((i, k))].clone()
                                * rhs.flattened[rhs.coord_to_idx((k, j))].clone();
                    }

                    elem
                };

                let flattened: Vec<T> = (0..(h1 * w2))
                    .map(|idx| calc_elem((idx / w2, idx % w2)))
                    .collect();

                Self {
                    shape: (h1, w2),
                    flattened,
                }
            }

            fn mat_mul_assign(&mut self, rhs: &Self) {
                *self = self.mat_mul(rhs);
            }
        }

        impl<T> Transpose<T> for Matrix<T>
        where
            T: Clone,
        {
            fn transposed(&self) -> Self {
                let mut flattened = vec![];

                for i in 0..self.elem_num() {
                    let coord = (i % self.shape.0, i / self.shape.0);
                    flattened.push(self.flattened[self.coord_to_idx(coord)].clone());
                }

                Self {
                    shape: (self.shape.1, self.shape.0),
                    flattened,
                }
            }
        }

        impl<T> MatPow<T> for Matrix<T>
        where
            T: Clone + Zero + One + MulAssign<T>,
        {
            fn mat_pow(&self, exp: usize) -> Self {
                assert!(self.is_square());

                let mut ret = Self::identity(self.shape.0);
                let mut mul = self.clone();
                let mut exp = exp;

                while exp != 0 {
                    if exp % 2 == 1 {
                        ret.mat_mul_assign(&mul);
                    }

                    mul = mul.mat_mul(&mul);
                    exp /= 2;
                }

                ret
            }
        }

        impl<T> Matrix<T>
        where
            T: Clone,
        {
            pub fn new(shape: (usize, usize)) -> Self
            where
                T: Default,
            {
                assert!(shape.0 >= 1 && shape.1 >= 1);

                Self {
                    shape,
                    flattened: vec![T::default(); shape.0 * shape.1],
                }
            }

            pub fn from_flattened(shape: (usize, usize), flattened: Vec<T>) -> Self {
                assert!(shape.0 >= 1 && shape.1 >= 1);
                assert_eq!(shape.0 * shape.1, flattened.len());

                Self { shape, flattened }
            }

            pub fn filled(shape: (usize, usize), x: T) -> Self {
                Self::from_flattened(shape, vec![x.clone(); shape.0 * shape.1])
            }

            pub fn zero(shape: (usize, usize)) -> Self
            where
                T: Zero,
            {
                Self::from_flattened(shape, vec![T::zero(); shape.0 * shape.1])
            }

            pub fn one(shape: (usize, usize)) -> Self
            where
                T: One,
            {
                Self::from_flattened(shape, vec![T::one(); shape.0 * shape.1])
            }

            pub fn identity(n: usize) -> Self
            where
                T: Zero + One,
            {
                let flattened = (0..n.pow(2))
                    .map(|elem_idx| {
                        let (i, j) = (elem_idx / n, elem_idx % n);

                        if i == j {
                            T::one()
                        } else {
                            T::zero()
                        }
                    })
                    .collect();

                Self::from_flattened((n, n), flattened)
            }

            pub fn from_vector(vec: Vec<T>) -> Self {
                Self {
                    shape: (vec.len(), 1),
                    flattened: vec,
                }
            }

            pub fn shape(&self) -> (usize, usize) {
                self.shape
            }

            pub fn flattened(self) -> Vec<T> {
                self.flattened
            }

            pub fn elem_num(&self) -> usize {
                self.shape.0 * self.shape.1
            }

            pub fn to_vec(self) -> Vec<Vec<T>> {
                let (h, w) = self.shape;

                let mut mat = vec![vec![]; h];
                mat.iter_mut().for_each(|x| x.reserve(w));

                for (i, elem) in self.flattened.into_iter().enumerate() {
                    mat[i / w].push(elem)
                }

                mat
            }

            #[allow(unused)]
            fn coord_to_idx(&self, coord: (usize, usize)) -> usize {
                debug_assert!(coord.0 < self.shape.0 && coord.1 < self.shape.1);

                coord.0 * self.shape.1 + coord.1
            }

            #[allow(unused)]
            fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
                debug_assert!(idx < self.elem_num());

                (idx / self.shape.1, idx % self.shape.1)
            }

            pub fn get(&self, coord: (usize, usize)) -> &T {
                let idx = self.coord_to_idx(coord);

                &self.flattened[idx]
            }

            pub fn set(&mut self, coord: (usize, usize), val: T) {
                let idx = self.coord_to_idx(coord);

                self.flattened[idx] = val;
            }

            pub fn is_square(&self) -> bool {
                self.shape.0 == self.shape.1
            }

            pub fn apply_to(&self, vec: &Vector<T>) -> Vector<T>
            where
                T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
            {
                let (h, w) = self.shape;

                assert_eq!(w, vec.len());

                let calc_elem = |i: usize| {
                    let mut elem = self.get((i, 0)).clone() * vec.get(0).clone();

                    for j in 1..w {
                        elem = elem + self.get((i, j)).clone() * vec.get(j).clone();
                    }

                    elem
                };

                (0..h).map(|i| calc_elem(i)).collect::<Vec<T>>().into()
            }
        }

        impl<T> From<Vec<Vec<T>>> for Matrix<T>
        where
            T: Clone,
        {
            fn from(mat: Vec<Vec<T>>) -> Self {
                let h = mat.len();
                assert_ne!(h, 0);

                let w = mat[0].len();
                assert_ne!(w, 0);

                assert!(mat.iter().all(|x| x.len() == w));

                Self {
                    shape: (h, w),
                    flattened: mat.into_iter().flatten().collect(),
                }
            }
        }

        impl<T> Add<&Matrix<T>> for &Matrix<T>
        where
            T: Clone + Add<T, Output = T>,
        {
            type Output = Matrix<T>;

            fn add(self, rhs: &Matrix<T>) -> Self::Output {
                assert_eq!(self.shape, rhs.shape);

                let flattened = self
                    .flattened
                    .iter()
                    .zip(rhs.flattened.iter())
                    .map(|(x, y)| x.clone() + y.clone())
                    .collect();

                Matrix {
                    shape: self.shape,
                    flattened,
                }
            }
        }

        impl<T> AddAssign<&Matrix<T>> for Matrix<T>
        where
            T: Clone + AddAssign<T>,
        {
            fn add_assign(&mut self, rhs: &Matrix<T>) {
                self.flattened
                    .iter_mut()
                    .zip(rhs.flattened.iter())
                    .for_each(|(x, y)| *x += y.clone());
            }
        }

        impl<T> Sub<&Matrix<T>> for &Matrix<T>
        where
            T: Clone + Sub<T, Output = T>,
        {
            type Output = Matrix<T>;

            fn sub(self, rhs: &Matrix<T>) -> Self::Output {
                assert_eq!(self.shape, rhs.shape);

                let flattened = self
                    .flattened
                    .iter()
                    .zip(rhs.flattened.iter())
                    .map(|(x, y)| x.clone() - y.clone())
                    .collect();

                Self::Output {
                    shape: self.shape,
                    flattened,
                }
            }
        }

        impl<T> SubAssign<&Matrix<T>> for Matrix<T>
        where
            T: Clone + SubAssign<T>,
        {
            fn sub_assign(&mut self, rhs: &Matrix<T>) {
                self.flattened
                    .iter_mut()
                    .zip(rhs.flattened.iter())
                    .for_each(|(x, y)| *x -= y.clone());
            }
        }

        impl<T> Mul<&Matrix<T>> for &Matrix<T>
        where
            T: Clone + Mul<T, Output = T>,
        {
            type Output = Matrix<T>;

            fn mul(self, rhs: &Matrix<T>) -> Self::Output {
                assert_eq!(self.shape, rhs.shape);

                let flattened: Vec<T> = self
                    .flattened
                    .iter()
                    .zip(rhs.flattened.iter())
                    .map(|(x, y)| x.clone() * y.clone())
                    .collect();

                Self::Output {
                    shape: self.shape,
                    flattened,
                }
            }
        }

        impl<T> MulAssign<&Matrix<T>> for Matrix<T>
        where
            T: Clone + MulAssign<T>,
        {
            fn mul_assign(&mut self, rhs: &Matrix<T>) {
                assert_eq!(self.shape, rhs.shape);

                self.flattened
                    .iter_mut()
                    .zip(rhs.flattened.iter())
                    .for_each(|(x, y)| *x *= y.clone());
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Vector<T>
        where
            T: Clone,
        {
            elements: Vec<T>,
        }

        pub trait InnerProduct<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn inner_product(&self, rhs: &Vector<T>) -> T;
        }

        impl<T> InnerProduct<T> for Vector<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn inner_product(&self, rhs: &Vector<T>) -> T {
                assert_eq!(self.len(), rhs.len());

                let mut ret = self.get(0).clone() * rhs.get(0).clone();

                for i in 1..self.len() {
                    ret = ret + self.get(i).clone() * rhs.get(i).clone();
                }

                ret
            }
        }

        impl<T> Vector<T>
        where
            T: Clone,
        {
            pub fn new(n: usize) -> Self
            where
                T: Default,
            {
                assert_ne!(n, 0);

                vec![T::default(); n].into()
            }

            pub fn len(&self) -> usize {
                self.elements.len()
            }

            pub fn elements(&self) -> &Vec<T> {
                &self.elements
            }

            pub fn to_vec(self) -> Vec<T> {
                self.elements
            }

            pub fn zero(n: usize) -> Self
            where
                T: Zero,
            {
                vec![T::zero(); n].into()
            }

            pub fn one(n: usize) -> Self
            where
                T: One,
            {
                vec![T::one(); n].into()
            }

            pub fn filled(x: T, n: usize) -> Self {
                vec![x.clone(); n].into()
            }

            pub fn get(&self, idx: usize) -> &T {
                &self.elements[idx]
            }

            pub fn apply_from(&self, mat: &Matrix<T>) -> Vector<T>
            where
                T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
            {
                let (h, w) = mat.shape;

                assert_eq!(self.len(), w);

                let calc_elem = |i: usize| {
                    let mut elem = mat.get((i, 0)).clone() * self.get(0).clone();

                    for j in 1..w {
                        elem = elem + mat.get((i, j)).clone() * self.get(j).clone();
                    }

                    elem
                };

                (0..h).map(|i| calc_elem(i)).collect::<Vec<T>>().into()
            }
        }

        impl<T> From<Vec<T>> for Vector<T>
        where
            T: Clone,
        {
            fn from(elements: Vec<T>) -> Self {
                assert!(!elements.is_empty());

                Self { elements }
            }
        }

        impl<T> Add<&Vector<T>> for &Vector<T>
        where
            T: Clone + Add<T, Output = T>,
        {
            type Output = Vector<T>;

            fn add(self, rhs: &Vector<T>) -> Self::Output {
                let elements: Vec<T> = self
                    .elements
                    .iter()
                    .zip(rhs.elements.iter())
                    .map(|(x, y)| x.clone() + y.clone())
                    .collect();

                Self::Output { elements }
            }
        }

        impl<T> AddAssign<&Vector<T>> for Vector<T>
        where
            T: Clone + AddAssign<T>,
        {
            fn add_assign(&mut self, rhs: &Vector<T>) {
                self.elements
                    .iter_mut()
                    .zip(rhs.elements.iter())
                    .for_each(|(x, y)| *x += y.clone());
            }
        }

        impl<T> Sub<&Vector<T>> for &Vector<T>
        where
            T: Clone + Sub<T, Output = T>,
        {
            type Output = Vector<T>;

            fn sub(self, rhs: &Vector<T>) -> Self::Output {
                let elements: Vec<T> = self
                    .elements
                    .iter()
                    .zip(rhs.elements.iter())
                    .map(|(x, y)| x.clone() - y.clone())
                    .collect();

                Self::Output { elements }
            }
        }

        impl<T> SubAssign<&Vector<T>> for Vector<T>
        where
            T: Clone + SubAssign<T>,
        {
            fn sub_assign(&mut self, rhs: &Vector<T>) {
                self.elements
                    .iter_mut()
                    .zip(rhs.elements.iter())
                    .for_each(|(x, y)| *x -= y.clone());
            }
        }

        impl<T> Mul<&Vector<T>> for &Vector<T>
        where
            T: Clone + Mul<T, Output = T>,
        {
            type Output = Vector<T>;

            fn mul(self, rhs: &Vector<T>) -> Self::Output {
                let elements: Vec<T> = self
                    .elements
                    .iter()
                    .zip(rhs.elements.iter())
                    .map(|(x, y)| x.clone() * y.clone())
                    .collect();

                Self::Output { elements }
            }
        }

        impl<T> MulAssign<&Vector<T>> for Vector<T>
        where
            T: Clone + MulAssign<T>,
        {
            fn mul_assign(&mut self, rhs: &Vector<T>) {
                self.elements
                    .iter_mut()
                    .zip(rhs.elements.iter())
                    .for_each(|(x, y)| *x *= y.clone());
            }
        }
    }
}

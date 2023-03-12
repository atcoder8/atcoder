use proconio::input;

use crate::atcoder8_library::matrix::{MatPowMod, Matrix};

fn main() {
    input! {
        (a, x, m): (usize, usize, usize),
    }

    let mat = Matrix::from(vec![vec![a, 1], vec![0, 1]]);

    let ans = *mat.mat_pow_mod(x, m).get((0, 1));
    println!("{}", ans);
}

pub mod atcoder8_library {
    pub mod matrix {
        use std::ops::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
    
        use num::{One, Zero};
    
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Matrix<T>
        where
            T: Clone,
        {
            shape: (usize, usize),
            flattened: Vec<T>,
        }
    
        pub trait Transpose<T>
        where
            T: Clone,
        {
            fn transposed(&self) -> Self;
        }
    
        pub trait Identity<T>
        where
            T: Clone + One + Zero,
        {
            fn identity(n: usize) -> Self;
        }
    
        pub trait MatMul<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn mat_mul(&self, rhs: &Self) -> Self;
        }
    
        pub trait MatMulAssign<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn mat_mul_assign(&mut self, rhs: &Self);
        }
    
        pub trait MatPow<T>
        where
            T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn mat_pow(&self, exp: usize) -> Self;
        }
    
        pub trait MatMulMod<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
        {
            fn mat_mul_mod(&self, rhs: &Self, modulus: T) -> Self;
        }
    
        pub trait MatMulAssignMod<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
        {
            fn mat_mul_assign_mod(&mut self, rhs: &Self, modulus: T);
        }
    
        pub trait MatPowMod<T>
        where
            T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
        {
            fn mat_pow_mod(&self, exp: usize, modulus: T) -> Self;
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
    
        impl<T> Identity<T> for Matrix<T>
        where
            T: Clone + Zero + One,
        {
            fn identity(n: usize) -> Self {
                let mut flattened = vec![T::zero(); n * n];
                for i in 0..n {
                    flattened[n * i + i] = T::one();
                }
    
                Self {
                    shape: (n, n),
                    flattened,
                }
            }
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
    
                    let init = self.get((i, 0)).clone() * rhs.get((0, j)).clone();
    
                    (1..w1)
                        .map(|k| self.get((i, k)).clone() * rhs.get((k, j)).clone())
                        .fold(init, |acc, x| acc + x)
                };
    
                let flattened: Vec<T> = (0..(h1 * w2))
                    .map(|idx| calc_elem((idx / w2, idx % w2)))
                    .collect();
    
                Self {
                    shape: (h1, w2),
                    flattened,
                }
            }
        }
    
        impl<T> MatMulAssign<T> for Matrix<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn mat_mul_assign(&mut self, rhs: &Self) {
                *self = self.mat_mul(rhs);
            }
        }
    
        impl<T> MatPow<T> for Matrix<T>
        where
            T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T>,
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
    
        impl<T> MatMulMod<T> for Matrix<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
        {
            fn mat_mul_mod(&self, rhs: &Self, modulus: T) -> Self {
                let (h1, w1) = self.shape;
                let (h2, w2) = rhs.shape;
    
                assert_eq!(w1, h2);
    
                let lhs = self.clone() % modulus.clone();
                let rhs = rhs.clone() % modulus.clone();
    
                let calc_elem = |coord: (usize, usize)| {
                    let (i, j) = coord;
    
                    let init = self.get((i, 0)).clone() * rhs.get((0, j)).clone() % modulus.clone();
    
                    let elem = (1..w1)
                        .map(|k| lhs.get((i, k)).clone() * rhs.get((k, j)).clone() % modulus.clone())
                        .fold(init, |acc, x| (acc + x) % modulus.clone());
    
                    (elem + modulus.clone()) % modulus.clone()
                };
    
                let flattened: Vec<T> = (0..(h1 * w2))
                    .map(|idx| calc_elem((idx / w2, idx % w2)))
                    .collect();
    
                Self {
                    shape: (h1, w2),
                    flattened,
                }
            }
        }
    
        impl<T> MatMulAssignMod<T> for Matrix<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
        {
            fn mat_mul_assign_mod(&mut self, rhs: &Self, modulus: T) {
                *self = self.mat_mul_mod(rhs, modulus);
            }
        }
    
        impl<T> MatPowMod<T> for Matrix<T>
        where
            T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
        {
            fn mat_pow_mod(&self, exp: usize, modulus: T) -> Self {
                assert!(self.is_square());
    
                let mut ret = Self::identity(self.shape.0) % modulus.clone();
                let mut mul = self.clone();
                let mut exp = exp;
    
                while exp != 0 {
                    if exp % 2 == 1 {
                        ret.mat_mul_assign_mod(&mul, modulus.clone());
                    }
    
                    mul = mul.mat_mul_mod(&mul, modulus.clone());
                    exp /= 2;
                }
    
                ret
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
    
        impl<T> Rem<T> for Matrix<T>
        where
            T: Clone + Rem<T, Output = T>,
        {
            type Output = Matrix<T>;
    
            fn rem(self, rhs: T) -> Self::Output {
                let flattened: Vec<T> = self
                    .flattened
                    .iter()
                    .map(|x| x.clone() % rhs.clone())
                    .collect();
    
                Self::Output {
                    shape: self.shape,
                    flattened,
                }
            }
        }
    
        impl<T> RemAssign<T> for Matrix<T>
        where
            T: Clone + RemAssign<T>,
        {
            fn rem_assign(&mut self, rhs: T) {
                self.flattened.iter_mut().for_each(|x| *x %= rhs.clone());
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
    
            pub fn get_mut(&mut self, coord: (usize, usize)) -> &mut T {
                let idx = self.coord_to_idx(coord);
    
                &mut self.flattened[idx]
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
    }
}

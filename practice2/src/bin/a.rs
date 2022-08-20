#[allow(unused_imports)]
use data_structures::*;

fn main() {
    let (n, q): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        let (t, u, v): (usize, usize, usize) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        };

        if t == 0 {
            uf.merge(u, v);
        } else {
            println!("{}", if uf.same(u, v) { 1 } else { 0 });
        }
    }
}

// This module is a collection of data structures.
pub mod data_structures {
    // Calculate ceil(log(2, x)).
    // If 0 is given, returns 0.
    pub fn ceil_log2(x: usize) -> usize {
        for i in 0.. {
            if (1 << i) >= x {
                return i;
            }
            if (1 << i) > usize::MAX / 2 {
                return i + 1;
            }
        }
        panic!()
    }

    pub struct UnionFind {
        parents: Vec<i32>,
        group_num: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            UnionFind {
                parents: vec![-1; n],
                group_num: n,
            }
        }

        pub fn leader(&mut self, x: usize) -> usize {
            if self.parents[x] < 0 {
                return x;
            }
            self.parents[x] = self.leader(self.parents[x] as usize) as i32;
            self.parents[x] as usize
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.leader(x) == self.leader(y)
        }

        pub fn merge(&mut self, x: usize, y: usize) -> bool {
            let mut leader_x = self.leader(x);
            let mut leader_y = self.leader(y);
            if leader_x == leader_y {
                return false;
            }
            if self.parents[leader_x] > self.parents[leader_y] {
                std::mem::swap(&mut leader_x, &mut leader_y);
            }
            self.parents[leader_x] += self.parents[leader_y];
            self.parents[leader_y] = leader_x as i32;
            self.group_num -= 1;
            true
        }

        pub fn size(&mut self, x: usize) -> usize {
            let leader_x = self.leader(x);
            -self.parents[leader_x] as usize
        }

        pub fn add(&mut self) {
            self.parents.push(-1);
            self.group_num += 1;
        }

        pub fn get_group_num(&self) -> usize {
            self.group_num
        }
    }

    pub trait Monoid {
        type S: Clone;
        fn e() -> Self::S;
        fn op(a: &Self::S, b: &Self::S) -> Self::S;
    }

    pub struct SegTree<M: Monoid> {
        n: usize,
        data: Vec<M::S>,
    }

    impl<M: Monoid> From<Vec<M::S>> for SegTree<M> {
        fn from(vec: Vec<M::S>) -> Self {
            let mut segtree = Self::new(vec.len());
            vec.into_iter()
                .enumerate()
                .for_each(|(i, x)| segtree.set(i, x));
            segtree
        }
    }

    impl<M: Monoid> SegTree<M> {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                data: vec![M::e(); 2 << ceil_log2(n)],
            }
        }

        pub fn set(&mut self, p: usize, x: M::S) {
            assert!(p < self.n);

            let mut p = p + self.data.len() / 2;
            self.data[p] = x;
            while p != 1 {
                p >>= 1;
                self.data[p] = M::op(&self.data[2 * p], &self.data[2 * p + 1]);
            }
        }

        pub fn get(&self, p: usize) -> M::S {
            assert!(p < self.n);

            self.data[self.data.len() / 2 + p].clone()
        }

        pub fn prod(&self, l: usize, r: usize) -> M::S {
            assert!(l <= r && r <= self.n);

            let mut sml = M::e();
            let mut smr = M::e();

            let mut l = l + self.data.len() / 2;
            let mut r = r + self.data.len() / 2;

            while l < r {
                if l & 1 != 0 {
                    sml = M::op(&sml, &self.data[l]);
                    l += 1;
                }

                if r & 1 != 0 {
                    r -= 1;
                    smr = M::op(&self.data[r], &smr);
                }

                l >>= 1;
                r >>= 1;
            }

            M::op(&sml, &smr)
        }

        pub fn all_prod(&self) -> M::S {
            self.data[1].clone()
        }

        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(f(&M::e()));

            if l == self.n {
                return self.n;
            }

            let size = self.data.len() / 2;

            let mut l = l + size;
            let mut sm = M::e();

            loop {
                while l % 2 == 0 {
                    l >>= 1;
                }

                if !f(&M::op(&sm, &self.data[l])) {
                    while l < size {
                        l *= 2;
                        let res = M::op(&sm, &self.data[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }

                    return l - size;
                }

                sm = M::op(&sm, &self.data[l]);
                l += 1;

                if l & (!l).wrapping_add(1) == l {
                    return self.n;
                }
            }
        }

        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(f(&M::e()));

            if r == 0 {
                return 0;
            }

            let size = self.data.len() / 2;

            let mut r = r + size;
            let mut sm = M::e();

            loop {
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }

                if !f(&M::op(&self.data[r], &sm)) {
                    while r < size {
                        r = 2 * r + 1;
                        let res = M::op(&self.data[r], &sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }

                    return r + 1 - size;
                }

                sm = M::op(&self.data[r], &sm);

                if r & (!r).wrapping_add(1) == r {
                    return 0;
                }
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
        ($modint_type:tt, $mod:literal) => {
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct $modint_type {
                val: u32,
            }

            impl $modint_type {
                const MOD: u32 = $mod;
            }

            impl $modint_type {
                pub fn new<T: RemEuclidU32>(val: T) -> Self {
                    Self {
                        val: val.rem_euclid_u32($mod),
                    }
                }

                pub fn raw(val: u32) -> Self {
                    Self { val }
                }

                pub fn get_val(&self) -> u32 {
                    self.val
                }

                pub fn inv(&self) -> Self {
                    Self::new(modinv(self.val, $mod))
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
                    Self::new(self.val + $mod - rhs.val)
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

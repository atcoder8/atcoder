use itertools::{iproduct, Itertools};
use modint2::Modint998244353;
use proconio::{input, marker::Chars};

use crate::union_find::UnionFind;

type Mint = Modint998244353;

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let coord_to_idx = |coord: (usize, usize)| w * coord.0 + coord.1;

    let mut uf = UnionFind::new(h * w);
    let mut red_num = 0;
    for (i, j) in iproduct!(0..h, 0..w) {
        if sss[i][j] != '#' {
            red_num += 1;
            continue;
        }

        let idx = coord_to_idx((i, j));
        if i < h - 1 && sss[i + 1][j] == '#' {
            uf.merge(idx, idx + w);
        }
        if j < w - 1 && sss[i][j + 1] == '#' {
            uf.merge(idx, idx + 1);
        }
    }

    let green_comp_num = uf.group_num() - red_num;
    let mut ans = Mint::new(green_comp_num);
    let inv_red_num = Mint::new(red_num).inv();
    for (row, col) in iproduct!(0..h, 0..w) {
        if sss[row][col] != '.' {
            continue;
        }

        let mut green_leaders = vec![];
        for (diff_row, diff_col) in DIFFS {
            let (next_row, next_col) = (row.wrapping_add(diff_row), col.wrapping_add(diff_col));

            if next_row >= h || next_col >= w {
                continue;
            }

            if sss[next_row][next_col] == '#' {
                let next_idx = coord_to_idx((next_row, next_col));
                let leader = uf.leader(next_idx);
                green_leaders.push(leader);
            }
        }

        if green_leaders.is_empty() {
            ans += inv_red_num;
        } else {
            ans -= inv_red_num * (green_leaders.iter().unique().count() - 1);
        }
    }

    println!("{}", ans);
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

pub mod union_find {
    //! Union-Find processes the following queries on undirected graphs.
    //! * Merge two connected components.
    //! * Determine whether two given nodes are in the same connected component.
    //!
    //! To seed up processing, merge optimization using the number of nodes
    //! of the connected components and path compression are performed.
    //!
    //! The time complexity of each query is `O(A(n))`.
    //! where `n` is the number of nodes in the graph and
    //! `A(n)` is the inverse of the Ackermann function.

    /// This is the value that will be associated with each nodes of the graph.
    #[derive(Debug, Clone, Copy)]
    enum ParentOrSize {
        /// It is used for non-representative nodes and stores the parent node.
        Parent(usize),

        /// It is used for the representative node and
        /// stores the number of nodes of the connected component.
        Size(usize),
    }

    /// Union-Find processes the following queries on undirected graphs.
    /// * Merge two connected components.
    /// * Determine whether two given nodes are in the same connected component.
    ///
    /// To seed up processing, merge optimization using the number of nodes
    /// of the connected components and path compression are performed.
    ///
    /// The time complexity of each query is `O(A(n))`.
    /// where `n` is the number of nodes in the graph and
    /// `A(n)` is the inverse of the Ackermann function.
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder8_library::union_find::UnionFind;
    ///
    /// let mut uf = UnionFind::new(3);
    /// assert_eq!(uf.same(0, 2), false);
    /// uf.merge(0, 1);
    /// assert_eq!(uf.same(0, 2), false);
    /// uf.merge(1, 2);
    /// assert_eq!(uf.same(0, 2), true);
    /// ```
    #[derive(Debug, Default, Clone)]
    pub struct UnionFind {
        /// For each node, one of the following is stored.
        /// * The number of nodes of the connected component to which it belongs.
        /// (If it is a representative node of the connected component.)
        /// * Index of the parent node. (Otherwise.)
        parent_or_size: Vec<ParentOrSize>,

        /// Number of connected components.
        group_num: usize,
    }

    impl UnionFind {
        /// Create an undirected graph with `n` nodes and `0` edges.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(0, 1);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(2, 1);
        /// assert_eq!(uf.same(0, 2), true);
        /// ```
        pub fn new(n: usize) -> Self {
            UnionFind {
                parent_or_size: vec![ParentOrSize::Size(1); n],
                group_num: n,
            }
        }

        /// Return the representative node of the connected component containing node `a`.
        ///
        /// At that time, perform path compression on the nodes on the path from node `a` to the representative node.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// uf.merge(1, 2);
        /// assert_eq!(uf.leader(0), 0);
        /// assert_eq!(uf.leader(1), uf.leader(2));
        /// ```
        pub fn leader(&mut self, a: usize) -> usize {
            // If node `a` is a representative node of the connected component, return `a`.
            if let ParentOrSize::Size(_) = self.parent_or_size[a] {
                return a;
            }

            // Path from node `a` to the representative node.
            let mut path = vec![];

            // Current node.
            let mut current = a;

            // Record the path to the representative node.
            while let ParentOrSize::Parent(parent) = self.parent_or_size[current] {
                // Add current node to the path.
                path.push(current);

                // Move to the parent node.
                current = parent;
            }

            // The representative node of the connected component.
            let leader = current;

            // Set nodes on the path as direct children of the representative node.
            path.iter().for_each(|&node| {
                self.parent_or_size[node] = ParentOrSize::Parent(leader);
            });

            // Return the representative node of the connected component.
            leader
        }

        /// Return whether two nodes `a` and `b` are in the same connected component.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(0, 1);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(2, 1);
        /// assert_eq!(uf.same(0, 2), true);
        /// ```
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.leader(a) == self.leader(b)
        }

        /// Merge each connected component containing nodes `a` and `b`.
        ///
        /// Return `true` if different connected components are newly merged.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(0, 1);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(2, 1);
        /// assert_eq!(uf.same(0, 2), true);
        /// ```
        pub fn merge(&mut self, a: usize, b: usize) -> bool {
            // Representative node of the connected component that contains the node `a`.
            let leader_a = self.leader(a);
            // Representative node of the connected component that contains the node `b`.
            let leader_b = self.leader(b);

            // If nodes `a` and `b` are in the same connected component, return `false` without processing.
            if leader_a == leader_b {
                return false;
            }

            // Number of nodes of the component containing node `a`.
            let component_size_a = self.size(leader_a);

            // Number of nodes of the component containing node `b`.
            let component_size_b = self.size(leader_b);

            // Number of nodes of the merged component.
            let merged_component_size = component_size_a + component_size_b;

            // Set the parent of the representative node of the smaller sized connected component
            // to be the parent of the other connected component.
            if component_size_a <= component_size_b {
                self.parent_or_size[leader_a] = ParentOrSize::Parent(leader_b);
                self.parent_or_size[leader_b] = ParentOrSize::Size(merged_component_size);
            } else {
                self.parent_or_size[leader_b] = ParentOrSize::Parent(leader_a);
                self.parent_or_size[leader_a] = ParentOrSize::Size(merged_component_size);
            }

            // Decrease the number of connected components by one.
            self.group_num -= 1;

            // Return `true` because different connected components are newly combined.
            true
        }

        /// Return a list of connected components.
        ///
        /// Each connected component consists of indexes of nodes.
        /// The indexes of the nodes in each connected component are arranged in ascending order.
        /// The list of connected components is sorted in ascending order
        /// with respect to the smallest index of the included nodes.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(5);
        /// uf.merge(1, 2);
        /// uf.merge(2, 3);
        /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3], vec![4]]);
        /// ```
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            // Number of nodes in graph.
            let element_num = self.parent_or_size.len();

            // List of connected components.
            let mut groups: Vec<Vec<usize>> = vec![];
            // Correspondence between the representative node and group index.
            let mut leader_to_idx: Vec<Option<usize>> = vec![None; element_num];

            // Assign each node in the graph to a group.
            for node in 0..element_num {
                // Representative node of the connected component to which the `node` belongs.
                let leader = self.leader(node);

                if let Some(group_idx) = leader_to_idx[leader] {
                    // Assign to an existing group.
                    groups[group_idx].push(node);
                } else {
                    // Adding a new group.
                    leader_to_idx[leader] = Some(groups.len());
                    groups.push(vec![node]);
                }
            }

            // Return a list of groups.
            groups
        }

        /// Return the number of nodes in the connected component to which node `a` belongs.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// assert_eq!(uf.size(0), 1);
        /// uf.merge(0, 1);
        /// assert_eq!(uf.size(0), 2);
        /// uf.merge(2, 1);
        /// assert_eq!(uf.size(0), 3);
        /// ```
        pub fn size(&mut self, a: usize) -> usize {
            let leader = self.leader(a);

            match self.parent_or_size[leader] {
                ParentOrSize::Parent(_) => panic!(),
                ParentOrSize::Size(size) => size,
            }
        }

        /// Add a new node with degree `0`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(4);
        /// uf.merge(1, 2);
        /// uf.merge(2, 3);
        /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3]]);
        /// uf.add();
        /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3], vec![4]]);
        /// ```
        pub fn add(&mut self) {
            self.parent_or_size.push(ParentOrSize::Size(1));
            self.group_num += 1;
        }

        /// Return the number of connected components.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// assert_eq!(uf.group_num(), 3);
        /// uf.merge(0, 1);
        /// assert_eq!(uf.group_num(), 2);
        /// uf.merge(2, 1);
        /// assert_eq!(uf.group_num(), 1);
        /// ```
        pub fn group_num(&self) -> usize {
            self.group_num
        }

        /// Return the number of nodes in the graph.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(5);
        /// assert_eq!(uf.elem_num(), 5);
        /// ```
        pub fn elem_num(&self) -> usize {
            self.parent_or_size.len()
        }
    }
}

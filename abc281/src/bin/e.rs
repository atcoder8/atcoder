use itertools::join;
use proconio::input;

use crate::atcoder8_library::{binary_search::BinarySearchWithUsize, fenwick_tree::FenwickTree};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        aa: [usize; n],
    }

    let (zip, unzip) = coordinate_compression(&aa);

    let mut ft_for_cnt: FenwickTree<usize> = FenwickTree::new(n);
    let mut ft_for_val: FenwickTree<usize> = FenwickTree::new(n);

    for &z in zip.iter().take(m) {
        ft_for_cnt.add(z, 1);
        ft_for_val.add(z, unzip[z]);
    }

    let is_ok = |x: usize| ft_for_cnt.sum(0..x) >= k;
    let right = (1..=n).binary_search(is_ok, false).unwrap();
    let rem = ft_for_cnt.sum(0..right) - k;

    let mut ans = vec![ft_for_val.sum(0..right) - rem * unzip[right - 1]];

    for i in m..n {
        ft_for_cnt.add(zip[i], 1);
        ft_for_val.add(zip[i], unzip[zip[i]]);

        ft_for_cnt.sub(zip[i - m], 1);
        ft_for_val.sub(zip[i - m], unzip[zip[i - m]]);

        let is_ok = |x: usize| ft_for_cnt.sum(0..x) >= k;
        let right = (1..=n).binary_search(is_ok, false).unwrap();
        let rem = ft_for_cnt.sum(0..right) - k;

        ans.push(ft_for_val.sum(0..right) - rem * unzip[right - 1]);
    }

    println!("{}", join(&ans, " "));
}

/// Performs coordinate compression of `seq`.
///
/// The return value is a tuple of `zip` and `unzip`.
/// `zip` is a list of the number of smallest values in the whole sequence for each element.
/// `unzip` is a list of the values that appear in the number sequence in ascending order.
/// The `i`-th element of the original sequence can be restored by `unzip[zip[i]]`.
pub fn coordinate_compression<T>(seq: &Vec<T>) -> (Vec<usize>, Vec<T>)
where
    T: Clone + Ord,
{
    let mut unzip = seq.clone();
    unzip.sort_unstable();
    unzip.dedup();

    let zip: Vec<usize> = seq
        .iter()
        .map(|x| unzip.binary_search(x).unwrap() as usize)
        .collect();

    (zip, unzip)
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

    pub mod binary_search {
        //! Implements binary search for range represented by the Rust's built-in range type.

        use std::ops::{
            Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
        };

        macro_rules! impl_binary_search_with_integer {
            ($int_type: ident, $fn_name_for_inc: ident, $fn_name_for_dec: ident, $fn_name: ident, $trait_name: ident) => {
                fn $fn_name_for_inc<R, F>(rng: R, mut is_ok: F) -> Option<$int_type>
                where
                    R: RangeBounds<$int_type>,
                    F: FnMut($int_type) -> bool,
                {
                    let mut left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => {
                            if start == std::$int_type::MAX {
                                return None;
                            }

                            start + 1
                        }
                        std::ops::Bound::Unbounded => std::$int_type::MIN,
                    };

                    let mut right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => {
                            if end == std::$int_type::MAX {
                                if !is_ok(end) {
                                    return None;
                                }

                                end
                            } else {
                                end + 1
                            }
                        }
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$int_type::MAX,
                    };

                    if left >= right {
                        return None;
                    }

                    if is_ok(left) {
                        return Some(left);
                    }

                    if left + 1 == right || !is_ok(right - 1) {
                        return None;
                    }

                    while right - left > 1 {
                        let mid = left + (right - left) / 2;

                        if is_ok(mid) {
                            right = mid;
                        } else {
                            left = mid;
                        }
                    }

                    Some(right)
                }

                fn $fn_name_for_dec<R, F>(rng: R, mut is_ok: F) -> Option<$int_type>
                where
                    R: RangeBounds<$int_type>,
                    F: FnMut($int_type) -> bool,
                {
                    let mut left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => {
                            if start == std::$int_type::MAX {
                                return None;
                            }

                            start + 1
                        }
                        std::ops::Bound::Unbounded => std::$int_type::MIN,
                    };

                    let mut right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => {
                            if end == std::$int_type::MAX {
                                if is_ok(end) {
                                    return Some(end);
                                }

                                end
                            } else {
                                end + 1
                            }
                        }
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$int_type::MAX,
                    };

                    if left >= right {
                        return None;
                    }

                    if is_ok(right - 1) {
                        return Some(right - 1);
                    }

                    if left + 1 == right || !is_ok(left) {
                        return None;
                    }

                    while right - left > 1 {
                        let mid = left + (right - left) / 2;

                        if is_ok(mid) {
                            left = mid;
                        } else {
                            right = mid;
                        }
                    }

                    Some(left)
                }

                /// If `is_ok` is monotonically increasing, returns the smallest integer `x`
                /// that satisfies `is_ok(x) = true` as the value of `Some`.
                ///
                /// If `is_ok` is monotonically decreasing, returns the largest integer `x`
                /// that satisfies `is_ok(x) = true` as the value of `Some`.
                ///
                /// Returns `None` if no such integer exists in both of the above cases.
                ///
                /// # Arguments
                ///
                /// * `rng` - Domain of function `is_ok`.
                /// * `is_ok` - Monotonic function.
                /// * `dec` - Represents that `is_ok` is a monotonically decreasing function if true,
                /// or a monotonically increasing function if false.
                ///
                /// # Examples
                ///
                /// ```
                /// use atcoder8_library::binary_search::binary_search_with_i64;
                ///
                /// let is_ok = |x: i64| { x.pow(2) >= 400 };
                /// assert_eq!(binary_search_with_i64(0..100, is_ok, false), Some(20));
                ///
                /// let is_ok = |x: i64| { x.pow(2) >= 400 };
                /// assert_eq!(binary_search_with_i64(0..10, is_ok, false), None);
                ///
                /// let is_ok = |x: i64| { x.pow(3) < -8000 };
                /// assert_eq!(binary_search_with_i64(-100..0, is_ok, true), Some(-21));
                /// ```
                pub fn $fn_name<R, F>(rng: R, is_ok: F, dec: bool) -> Option<$int_type>
                where
                    R: RangeBounds<$int_type>,
                    F: FnMut($int_type) -> bool,
                {
                    if dec {
                        $fn_name_for_dec(rng, is_ok)
                    } else {
                        $fn_name_for_inc(rng, is_ok)
                    }
                }

                pub trait $trait_name: Sized + RangeBounds<$int_type> {
                    /// Performs a binary search on the domain specified by the Rust's built-in range type.
                    ///
                    /// If `is_ok` is monotonically increasing, returns the smallest integer `x`
                    /// that satisfies `is_ok(x) = true` as the value of `Some`.
                    ///
                    /// If `is_ok` is monotonically decreasing, returns the largest integer `x`
                    /// that satisfies `is_ok(x) = true` as the value of `Some`.
                    ///
                    /// Returns `None` if no such integer exists in both of the above cases.
                    ///
                    /// # Arguments
                    ///
                    /// * `is_ok` - Monotonic function.
                    /// * `dec` - Represents that `is_ok` is a monotonically decreasing function if true,
                    /// or a monotonically increasing function if false.
                    ///
                    /// # Examples
                    ///
                    /// ```
                    /// use atcoder8_library::binary_search::BinarySearchWithI64;
                    ///
                    /// let is_ok = |x: i64| { x.pow(2) >= 400 };
                    /// assert_eq!((0..100).binary_search(is_ok, false), Some(20));
                    ///
                    /// let is_ok = |x: i64| { x.pow(2) >= 400 };
                    /// assert_eq!((0..10).binary_search(is_ok, false), None);
                    ///
                    /// let is_ok = |x: i64| { x.pow(3) < -8000 };
                    /// assert_eq!((-100..0).binary_search(is_ok, true), Some(-21));
                    /// ```
                    fn binary_search<F>(self, is_ok: F, dec: bool) -> Option<$int_type>
                    where
                        F: FnMut($int_type) -> bool,
                    {
                        $fn_name(self, is_ok, dec)
                    }
                }

                impl $trait_name for RangeFull {}

                impl $trait_name for RangeTo<$int_type> {}

                impl $trait_name for RangeToInclusive<$int_type> {}

                impl $trait_name for RangeFrom<$int_type> {}

                impl $trait_name for Range<$int_type> {}

                impl $trait_name for RangeInclusive<$int_type> {}
            };
        }

        impl_binary_search_with_integer!(
            i8,
            binary_search_with_i8_for_inc,
            binary_search_with_i8_for_dec,
            binary_search_with_i8,
            BinarySearchWithI8
        );

        impl_binary_search_with_integer!(
            i16,
            binary_search_with_i16_for_inc,
            binary_search_with_i16_for_dec,
            binary_search_with_i16,
            BinarySearchWithI16
        );

        impl_binary_search_with_integer!(
            i32,
            binary_search_with_i32_for_inc,
            binary_search_with_i32_for_dec,
            binary_search_with_i32,
            BinarySearchWithI32
        );

        impl_binary_search_with_integer!(
            i64,
            binary_search_with_i64_for_inc,
            binary_search_with_i64_for_dec,
            binary_search_with_i64,
            BinarySearchWithI64
        );

        impl_binary_search_with_integer!(
            i128,
            binary_search_with_i128_for_inc,
            binary_search_with_i128_for_dec,
            binary_search_with_i128,
            BinarySearchWithI128
        );

        impl_binary_search_with_integer!(
            isize,
            binary_search_with_isize_for_inc,
            binary_search_with_isize_for_dec,
            binary_search_with_isize,
            BinarySearchWithIsize
        );

        impl_binary_search_with_integer!(
            u8,
            binary_search_with_u8_for_inc,
            binary_search_with_u8_for_dec,
            binary_search_with_u8,
            BinarySearchWithU8
        );

        impl_binary_search_with_integer!(
            u16,
            binary_search_with_u16_for_inc,
            binary_search_with_u16_for_dec,
            binary_search_with_u16,
            BinarySearchWithU16
        );

        impl_binary_search_with_integer!(
            u32,
            binary_search_with_u32_for_inc,
            binary_search_with_u32_for_dec,
            binary_search_with_u32,
            BinarySearchWithU32
        );

        impl_binary_search_with_integer!(
            u64,
            binary_search_with_u64_for_inc,
            binary_search_with_u64_for_dec,
            binary_search_with_u64,
            BinarySearchWithU64
        );

        impl_binary_search_with_integer!(
            u128,
            binary_search_with_u128_for_inc,
            binary_search_with_u128_for_dec,
            binary_search_with_u128,
            BinarySearchWithU128
        );

        impl_binary_search_with_integer!(
            usize,
            binary_search_with_usize_for_inc,
            binary_search_with_usize_for_dec,
            binary_search_with_usize,
            BinarySearchWithUsize
        );

        macro_rules! impl_binary_search_with_float {
            ($float_type: ident, $fn_name_for_inc: ident, $fn_name_for_dec: ident, $fn_name: ident, $trait_name: ident) => {
                fn $fn_name_for_inc<R, F>(
                    rng: R,
                    mut is_ok: F,
                    eps: $float_type,
                ) -> Option<$float_type>
                where
                    R: RangeBounds<$float_type>,
                    F: FnMut($float_type) -> bool,
                {
                    let mut left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => start,
                        std::ops::Bound::Unbounded => std::$float_type::MIN,
                    };

                    let mut right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => end,
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$float_type::MAX,
                    };

                    assert!(
                        eps > 0.0,
                        "Allowable margin of error must be a positive number."
                    );

                    if left >= right {
                        return None;
                    }

                    if is_ok(left) {
                        return Some(left);
                    }

                    if !is_ok(right) {
                        return None;
                    }

                    while right - left > eps {
                        let mid = right - (right - left) / 2.0;

                        if mid <= left || right <= mid {
                            return None;
                        }

                        if is_ok(mid) {
                            right = mid;
                        } else {
                            left = mid;
                        }
                    }

                    Some(right)
                }

                fn $fn_name_for_dec<R, F>(
                    rng: R,
                    mut is_ok: F,
                    eps: $float_type,
                ) -> Option<$float_type>
                where
                    R: RangeBounds<$float_type>,
                    F: FnMut($float_type) -> bool,
                {
                    let mut left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => start,
                        std::ops::Bound::Unbounded => std::$float_type::MIN,
                    };

                    let mut right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => end,
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$float_type::MAX,
                    };

                    assert!(
                        eps > 0.0,
                        "Allowable margin of error must be a positive number."
                    );

                    if left >= right {
                        return None;
                    }

                    if is_ok(right) {
                        return Some(right);
                    }

                    if !is_ok(left) {
                        return None;
                    }

                    while (right - left) > eps {
                        let mid = right - (right - left) / 2.0;

                        if mid <= left || right <= mid {
                            return None;
                        }

                        if is_ok(mid) {
                            left = mid;
                        } else {
                            right = mid;
                        }
                    }

                    Some(left)
                }

                /// If `is_ok` is monotonically increasing,
                /// returns the smallest floating point number `x`
                /// that satisfies `is_ok(x) = true` as the value of `Some`.
                ///
                /// If `is_ok` is monotonically decreasing,
                /// returns the largest floating point number `x`
                /// that satisfies `is_ok(x) = true` as the value of `Some`.
                ///
                /// Returns `None` if no such floating point number exists in both of the above cases.
                /// This includes the case where the absolute error cannot be determined
                /// to be less than or equal to `eps`.
                ///
                /// # Arguments
                ///
                /// * `rng` - Domain of function `is_ok`.
                /// * `is_ok` - Monotonic function.
                /// * `eps` - The allowable absolute error. It must be a positive number.
                /// * `dec` - Represents that `is_ok` is a monotonically decreasing function if true,
                /// or a monotonically increasing function if false.
                ///
                /// # Examples
                ///
                /// ```
                /// use atcoder8_library::binary_search::binary_search_with_f64;
                ///
                /// let is_ok = |x: f64| { x.powi(2) >= 400.0 };
                /// let ans = binary_search_with_f64(0.0..100.0, is_ok, 1e-6, false).unwrap();
                /// assert!((ans - 20.0).abs() <= 1e-6);
                ///
                /// let is_ok = |x: f64| { x.powi(2) >= 400.0 };
                /// assert_eq!(binary_search_with_f64(0.0..10.0, is_ok, 1e-6, false), None);
                ///
                /// let is_ok = |x: f64| { x.powi(3) <= -8000.0 };
                /// let ans = binary_search_with_f64(-100.0..0.0, is_ok, 1e-6, true).unwrap();
                /// assert!((ans - (-20.0)).abs() <= 1e-6);
                /// ```
                pub fn $fn_name<R, F>(
                    rng: R,
                    is_ok: F,
                    eps: $float_type,
                    dec: bool,
                ) -> Option<$float_type>
                where
                    R: RangeBounds<$float_type>,
                    F: FnMut($float_type) -> bool,
                {
                    if dec {
                        $fn_name_for_dec(rng, is_ok, eps)
                    } else {
                        $fn_name_for_inc(rng, is_ok, eps)
                    }
                }

                pub trait $trait_name: Sized + RangeBounds<$float_type> {
                    /// Performs a binary search on the domain specified by the Rust's built-in range type.
                    ///
                    /// If `is_ok` is monotonically increasing,
                    /// returns the smallest floating point number `x`
                    /// that satisfies `is_ok(x) = true` as the value of `Some`.
                    ///
                    /// If `is_ok` is monotonically decreasing,
                    /// returns the largest floating point number `x`
                    /// that satisfies `is_ok(x) = true` as the value of `Some`.
                    ///
                    /// Returns `None` if no such floating point number exists in both of the above cases.
                    /// This includes the case where the absolute error cannot be determined
                    /// to be less than or equal to `eps`.
                    ///
                    /// # Arguments
                    ///
                    /// * `is_ok` - Monotonic function.
                    /// * `eps` - The allowable absolute error. It must be a positive number.
                    /// * `dec` - Represents that `is_ok` is a monotonically decreasing function if true,
                    /// or a monotonically increasing function if false.
                    ///
                    /// # Examples
                    ///
                    /// ```
                    /// use atcoder8_library::binary_search::BinarySearchWithF64;
                    ///
                    /// let is_ok = |x: f64| { x.powi(2) >= 400.0 };
                    /// let ans = (0.0..100.0).binary_search(is_ok, 1e-6, false).unwrap();
                    /// assert!((ans - 20.0).abs() <= 1e-6);
                    ///
                    /// let is_ok = |x: f64| { x.powi(2) >= 400.0 };
                    /// assert_eq!((0.0..10.0).binary_search(is_ok, 1e-6, false), None);
                    ///
                    /// let is_ok = |x: f64| { x.powi(3) <= -8000.0 };
                    /// let ans = (-100.0..0.0).binary_search(is_ok, 1e-6, true).unwrap();
                    /// assert!((ans - (-20.0)).abs() <= 1e-6);
                    /// ```
                    fn binary_search<F>(
                        self,
                        is_ok: F,
                        eps: $float_type,
                        dec: bool,
                    ) -> Option<$float_type>
                    where
                        F: FnMut($float_type) -> bool,
                    {
                        $fn_name(self, is_ok, eps, dec)
                    }
                }

                impl $trait_name for RangeFull {}

                impl $trait_name for RangeTo<$float_type> {}

                impl $trait_name for RangeToInclusive<$float_type> {}

                impl $trait_name for RangeFrom<$float_type> {}

                impl $trait_name for Range<$float_type> {}

                impl $trait_name for RangeInclusive<$float_type> {}
            };
        }

        impl_binary_search_with_float!(
            f32,
            binary_search_with_f32_for_inc,
            binary_search_with_f32_for_dec,
            binary_search_with_f32,
            BinarySearchWithF32
        );

        impl_binary_search_with_float!(
            f64,
            binary_search_with_f64_for_inc,
            binary_search_with_f64_for_dec,
            binary_search_with_f64,
            BinarySearchWithF64
        );
    }
}

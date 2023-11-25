use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

use crate::binary_search::BinarySearchWithUsize;

fn main() {
    input! {
        (n, k): (usize, usize),
        xy: [(usize, usize); n],
    }

    let calc_cost = |mut coord_counts: VecDeque<(usize, usize)>, dest: usize| {
        let mut cost = 0;

        loop {
            let &(left_num, left) = coord_counts.front().unwrap();
            let &(right_num, right) = coord_counts.back().unwrap();
            let width = right - left;

            if width <= dest {
                break;
            }

            if left_num <= right_num {
                coord_counts.pop_front();

                let (second_left_num, second_left) = coord_counts.front_mut().unwrap();
                let reduce = (*second_left - left).min(width - dest);
                let reduced_left = left + reduce;

                if reduced_left == *second_left {
                    *second_left_num += left_num;
                } else {
                    coord_counts.push_front((left_num, reduced_left));
                }

                cost += left_num * reduce;
            } else {
                coord_counts.pop_back();

                let (second_right_num, second_right) = coord_counts.back_mut().unwrap();
                let reduce = (right - *second_right).min(width - dest);
                let reduced_right = right - reduce;

                if reduced_right == *second_right {
                    *second_right_num += right_num;
                } else {
                    coord_counts.push_back((right_num, reduced_right));
                }

                cost += right_num * reduce;
            }
        }

        cost
    };

    let is_ok = |dest: usize| {
        let hor: VecDeque<_> = xy
            .iter()
            .map(|&(x, _)| x)
            .sorted_unstable()
            .dedup_with_count()
            .collect();

        let ver: VecDeque<_> = xy
            .iter()
            .map(|&(_, y)| y)
            .sorted_unstable()
            .dedup_with_count()
            .collect();

        calc_cost(hor, dest) + calc_cost(ver, dest) <= k
    };

    let ans = (0..=10_usize.pow(9)).binary_search(is_ok, false).unwrap();
    println!("{}", ans);
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
            fn $fn_name_for_inc<R, F>(rng: R, mut is_ok: F, eps: $float_type) -> Option<$float_type>
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

            fn $fn_name_for_dec<R, F>(rng: R, mut is_ok: F, eps: $float_type) -> Option<$float_type>
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

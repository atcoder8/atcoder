use atcoder8_library::binary_search::BinarySearchWithUsize;

use proconio::input;

fn main() {
    input! {
        (n, l): (usize, usize),
        k: usize,
        mut aa: [usize; n],
    }

    aa.insert(0, 0);
    aa.push(l);

    let bb: Vec<usize> = aa.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let is_ok = |mid: usize| {
        let mut piece_cnt = 0;
        let mut len = 0;

        for &b in bb.iter() {
            len += b;
            if len >= mid {
                piece_cnt += 1;
                if piece_cnt == k + 1 {
                    return true;
                }
                len = 0;
            }
        }

        false
    };

    let ans = (0..l).binary_search(is_ok, true).unwrap();
    println!("{}", ans);
}

pub mod atcoder8_library {
    pub mod binary_search {
        //! Implements binary search for range represented by
        //! integer or floating point number primitive types.
    
        use std::ops::{
            Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
        };
    
        macro_rules! impl_binary_search_with_integer {
            ($int_type: ident, $fn_name_for_inc: ident, $fn_name_for_dec: ident, $fn_name: ident, $trait_name: ident) => {
                fn $fn_name_for_inc<R, F>(rng: R, is_ok: F) -> Option<$int_type>
                where
                    R: RangeBounds<$int_type>,
                    F: Fn($int_type) -> bool,
                {
                    let mut left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => start + 1,
                        std::ops::Bound::Unbounded => std::$int_type::MIN,
                    };
    
                    let right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => end + 1,
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$int_type::MAX,
                    };
    
                    assert!(left < right, "The interval represented by `rng` is empty.");
    
                    if is_ok(left) {
                        return Some(left);
                    }
    
                    if !is_ok(right - 1) {
                        return None;
                    }
    
                    let mut size = right - left;
    
                    while size > 1 {
                        let half = size / 2;
                        let mid = left + half;
    
                        if !is_ok(mid) {
                            left = mid;
                        }
                        size -= half;
                    }
    
                    let boundary = if is_ok(left) { left } else { left + 1 };
                    Some(boundary)
                }
    
                fn $fn_name_for_dec<R, F>(rng: R, is_ok: F) -> Option<$int_type>
                where
                    R: RangeBounds<$int_type>,
                    F: Fn($int_type) -> bool,
                {
                    let left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => start + 1,
                        std::ops::Bound::Unbounded => std::$int_type::MIN,
                    };
    
                    let right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => end + 1,
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$int_type::MAX,
                    };
    
                    assert!(left < right, "The interval represented by `rng` is empty.");
    
                    if is_ok(right - 1) {
                        return Some(right - 1);
                    }
    
                    if !is_ok(left) {
                        return None;
                    }
    
                    let boundary = $fn_name_for_inc(rng, |mid| !is_ok(mid)).unwrap() - 1;
                    Some(boundary)
                }
    
                /// Returns the smallest integer `x` in the range `rng` for which `is_ok(x) = true`.
                /// If no such integer exists, returns None.
                ///
                /// # Arguments
                ///
                /// * `rng` - Search range.
                /// * `is_ok` - Monotonic function.
                /// * `dec` - Indicates that `is_ok` is a monotonically decreasing function if true,
                /// or a monotonically increasing function if false.
                pub fn $fn_name<R, F>(rng: R, is_ok: F, dec: bool) -> Option<$int_type>
                where
                    R: RangeBounds<$int_type>,
                    F: Fn($int_type) -> bool,
                {
                    if dec {
                        $fn_name_for_dec(rng, is_ok)
                    } else {
                        $fn_name_for_inc(rng, is_ok)
                    }
                }
    
                pub trait $trait_name: Sized + RangeBounds<$int_type> {
                    /// Returns the smallest integer `x` in the range for which `is_ok(x) = true`.
                    /// If no such integer exists, returns None.
                    ///
                    /// # Arguments
                    ///
                    /// * `is_ok` - Monotonic function.
                    /// * `dec` - Indicates that `is_ok` is a monotonically decreasing function if true,
                    /// or a monotonically increasing function if false.
                    fn binary_search<F>(self, is_ok: F, dec: bool) -> Option<$int_type>
                    where
                        F: Fn($int_type) -> bool,
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
                fn $fn_name_for_inc<R, F>(rng: R, is_ok: F, eps: $float_type) -> Option<$float_type>
                where
                    R: RangeBounds<$float_type>,
                    F: Fn($float_type) -> bool,
                {
                    let mut left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => start,
                        std::ops::Bound::Unbounded => std::$float_type::MIN,
                    };
    
                    let right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => end,
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$float_type::MAX,
                    };
    
                    assert!(left < right, "The interval represented by `rng` is empty.");
    
                    assert!(
                        eps > 0.0,
                        "Allowable margin of error must be a positive number."
                    );
    
                    if is_ok(left) {
                        return Some(left);
                    }
    
                    if !is_ok(right) {
                        return None;
                    }
    
                    let mut size = right - left;
    
                    while size > eps {
                        let half = size / 2.0;
                        let mid = left + half;
    
                        if !is_ok(mid) {
                            left = mid;
                        }
                        size -= half;
                    }
    
                    Some(right)
                }
    
                fn $fn_name_for_dec<R, F>(rng: R, is_ok: F, eps: $float_type) -> Option<$float_type>
                where
                    R: RangeBounds<$float_type>,
                    F: Fn($float_type) -> bool,
                {
                    let left = match rng.start_bound() {
                        std::ops::Bound::Included(&start) => start,
                        std::ops::Bound::Excluded(&start) => start,
                        std::ops::Bound::Unbounded => std::$float_type::MIN,
                    };
    
                    let mut right = match rng.end_bound() {
                        std::ops::Bound::Included(&end) => end,
                        std::ops::Bound::Excluded(&end) => end,
                        std::ops::Bound::Unbounded => std::$float_type::MAX,
                    };
    
                    assert!(left < right, "The interval represented by `rng` is empty.");
    
                    assert!(
                        eps > 0.0,
                        "Allowable margin of error must be a positive number."
                    );
    
                    if is_ok(right) {
                        return Some(right);
                    }
    
                    if !is_ok(left) {
                        return None;
                    }
    
                    let mut size = right - left;
    
                    while size > eps {
                        let half = size / 2.0;
                        let mid = right - half;
    
                        if !is_ok(mid) {
                            right = mid;
                        }
                        size -= half;
                    }
    
                    Some(left)
                }
    
                /// Returns the smallest floating point number `x` in the range `rng` for which `is_ok(x) = true`.
                /// However, an error with the true value within `eps` is allowed.
                /// If no such floating point number exists, returns None.
                ///
                /// # Arguments
                ///
                /// * `rng` - Search range.
                /// * `is_ok` - Monotonic function.
                /// * `eps` - Allowable margin of error.
                /// * `dec` - Indicates that `is_ok` is a monotonically decreasing function if true,
                /// or a monotonically increasing function if false.
                pub fn $fn_name<R, F>(
                    rng: R,
                    is_ok: F,
                    eps: $float_type,
                    dec: bool,
                ) -> Option<$float_type>
                where
                    R: RangeBounds<$float_type>,
                    F: Fn($float_type) -> bool,
                {
                    if dec {
                        $fn_name_for_dec(rng, is_ok, eps)
                    } else {
                        $fn_name_for_inc(rng, is_ok, eps)
                    }
                }
    
                pub trait $trait_name: Sized + RangeBounds<$float_type> {
                    /// Returns the smallest floating point number `x` in the range for which `is_ok(x) = true`.
                    /// However, an error with the true value within `eps` is allowed.
                    /// If no such floating point number exists, returns None.
                    ///
                    /// # Arguments
                    ///
                    /// * `is_ok` - Monotonic function.
                    /// * `eps` - Allowable margin of error.
                    /// * `dec` - Indicates that `is_ok` is a monotonically decreasing function if true,
                    /// or a monotonically increasing function if false.
                    fn binary_search<F>(
                        self,
                        is_ok: F,
                        eps: $float_type,
                        dec: bool,
                    ) -> Option<$float_type>
                    where
                        F: Fn($float_type) -> bool,
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

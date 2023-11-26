use itertools::Itertools;
use lazy_segtree::{Mapping, Monoid};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

use crate::lazy_segtree::LazySegtree;

fn main() {
    input! {
        (_n, q): (usize, usize),
        s: Chars,
        queries: [(usize, Usize1, usize); q],
    }

    let seq = s
        .iter()
        .map(|&c| {
            if c == '0' {
                S {
                    left_zero: 1,
                    left_one: 0,
                    right_zero: 1,
                    right_one: 0,
                    max_zero: 1,
                    max_one: 0,
                    size: 1,
                }
            } else {
                S {
                    left_zero: 0,
                    left_one: 1,
                    right_zero: 0,
                    right_one: 1,
                    max_zero: 0,
                    max_one: 1,
                    size: 1,
                }
            }
        })
        .collect_vec();

    let mut segtree = LazySegtree::<S, F>::from(seq);
    for &(c, l, r) in &queries {
        if c == 1 {
            segtree.apply_range(l..r, F { flip: true })
        } else {
            let ans = segtree.product_range(l..r).max_one;
            println!("{}", ans);
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct S {
    left_zero: usize,
    left_one: usize,
    right_zero: usize,
    right_one: usize,
    max_zero: usize,
    max_one: usize,
    size: usize,
}

impl Monoid for S {
    fn id() -> Self {
        S::default()
    }

    fn product(&self, rhs: &Self) -> Self {
        let product_size = self.size + rhs.size;

        let max_max_zero = self.max_zero.max(rhs.max_zero);
        let (left_zero, right_zero, max_zero) =
            match (self.left_zero == self.size, rhs.left_zero == rhs.size) {
                (true, true) => (product_size, product_size, product_size),

                (true, false) => (
                    self.size + rhs.left_zero,
                    rhs.right_zero,
                    max_max_zero.max(self.size + rhs.left_zero),
                ),

                (false, true) => (
                    self.left_zero,
                    self.right_zero + rhs.size,
                    max_max_zero.max(self.right_zero + rhs.size),
                ),

                (false, false) => (
                    self.left_zero,
                    rhs.right_zero,
                    max_max_zero.max(self.right_zero + rhs.left_zero),
                ),
            };

        let max_max_one = self.max_one.max(rhs.max_one);
        let (left_one, right_one, max_one) =
            match (self.left_one == self.size, rhs.left_one == rhs.size) {
                (true, true) => (product_size, product_size, product_size),

                (true, false) => (
                    self.size + rhs.left_one,
                    rhs.right_one,
                    max_max_one.max(self.size + rhs.left_one),
                ),

                (false, true) => (
                    self.left_one,
                    self.right_one + rhs.size,
                    max_max_one.max(self.right_one + rhs.size),
                ),

                (false, false) => (
                    self.left_one,
                    rhs.right_one,
                    max_max_one.max(self.right_one + rhs.left_one),
                ),
            };

        Self {
            left_zero,
            left_one,
            right_zero,
            right_one,
            max_zero,
            max_one,
            size: product_size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct F {
    flip: bool,
}

impl Monoid for F {
    fn id() -> Self {
        Self { flip: false }
    }

    fn product(&self, rhs: &Self) -> Self {
        Self {
            flip: self.flip != rhs.flip,
        }
    }
}

impl Mapping<S> for F {
    fn mapping(&self, s: &S) -> S {
        if self.flip {
            S {
                left_zero: s.left_one,
                left_one: s.left_zero,
                right_zero: s.right_one,
                right_one: s.right_zero,
                max_zero: s.max_one,
                max_one: s.max_zero,
                size: s.size,
            }
        } else {
            *s
        }
    }
}

pub mod lazy_segtree {
    use std::ops::RangeBounds;

    pub trait Monoid {
        /// The identity element.
        fn id() -> Self;

        /// The binary operation.
        fn product(&self, rhs: &Self) -> Self;
    }

    pub trait Mapping<S>: Monoid
    where
        S: Monoid,
    {
        fn mapping(&self, s: &S) -> S;
    }

    #[derive(Debug, Default, Clone)]
    pub struct LazySegtree<S, F>
    where
        S: Monoid,
        F: Mapping<S>,
    {
        n: usize,
        log: u32,
        size: usize,
        data: Vec<S>,
        lazy: Vec<F>,
    }

    impl<S, F> From<Vec<S>> for LazySegtree<S, F>
    where
        S: Clone + Monoid,
        F: Clone + Mapping<S>,
    {
        fn from(value: Vec<S>) -> Self {
            if value.is_empty() {
                return Self {
                    n: 0,
                    log: 0,
                    size: 0,
                    data: vec![],
                    lazy: vec![],
                };
            }

            let n = value.len();
            let log = usize::BITS - (n - 1).leading_zeros();
            let size = 1 << log;
            let mut data = vec![S::id(); size];
            data.extend(value);
            data.extend(vec![S::id(); size - n]);

            let mut seg = Self {
                n,
                log,
                size,
                data,
                lazy: vec![F::id(); size],
            };

            for i in (1..size).rev() {
                seg.up_product(i);
            }

            seg
        }
    }

    impl<S, F> LazySegtree<S, F>
    where
        S: Monoid,
        F: Mapping<S>,
    {
        pub fn new(n: usize) -> Self
        where
            S: Clone,
            F: Clone,
        {
            if n == 0 {
                return Self {
                    n: 0,
                    log: 0,
                    size: 0,
                    data: vec![],
                    lazy: vec![],
                };
            }

            let log = usize::BITS - (n - 1).leading_zeros();
            let size = 1 << log;

            Self {
                n,
                log,
                size,
                data: vec![S::id(); 2 * size],
                lazy: vec![F::id(); size],
            }
        }

        fn get_bounds<R>(&self, rng: R) -> (usize, usize)
        where
            R: RangeBounds<usize>,
        {
            let l = match rng.start_bound() {
                std::ops::Bound::Included(&start_bound) => start_bound,
                std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let r = match rng.end_bound() {
                std::ops::Bound::Included(&end_bound) => end_bound + 1,
                std::ops::Bound::Excluded(&end_bound) => end_bound,
                std::ops::Bound::Unbounded => self.n,
            };

            (l, r)
        }

        fn up_product(&mut self, k: usize) {
            assert!(1 <= k && k < self.size);

            self.data[k] = self.data[2 * k].product(&self.data[2 * k + 1]);
        }

        fn down_composite(&mut self, k: usize) {
            assert!(1 <= k && k < self.size);

            let left_child = 2 * k;
            self.data[left_child] = self.lazy[k].mapping(&self.data[left_child]);

            let right_child = left_child + 1;
            self.data[right_child] = self.lazy[k].mapping(&self.data[right_child]);

            if left_child < self.size {
                self.lazy[left_child] = self.lazy[left_child].product(&self.lazy[k]);
                self.lazy[right_child] = self.lazy[right_child].product(&self.lazy[k]);
            }

            self.lazy[k] = F::id();
        }

        fn composite(&mut self, p: usize) {
            assert!(p < self.n);

            let p = p + self.size;

            for i in (1..=self.log).rev() {
                self.down_composite(p >> i);
            }
        }

        pub fn get(&mut self, p: usize) -> &S {
            self.composite(p);

            &self.data[p + self.size]
        }

        pub fn set(&mut self, p: usize, x: S) {
            self.composite(p);

            let p = p + self.size;

            self.data[p] = x;
            for i in 1..=self.log {
                self.up_product(p >> i);
            }
        }

        pub fn apply(&mut self, p: usize, f: F) {
            self.composite(p);

            let p = p + self.size;

            self.data[p] = f.mapping(&self.data[p]);
            for i in 1..=self.log {
                self.up_product(p >> i);
            }
        }

        fn composite_range<R>(&mut self, rng: R)
        where
            R: RangeBounds<usize>,
        {
            let (mut left, mut right) = self.get_bounds(rng);

            assert!(left <= right && right <= self.n);

            left += self.size;
            right += self.size;

            for i in (left.trailing_zeros() + 1..=self.log).rev() {
                self.down_composite(left >> i);
            }

            for i in (right.trailing_zeros() + 1..=self.log).rev() {
                self.down_composite((right - 1) >> i);
            }
        }

        pub fn product_range<R>(&mut self, rng: R) -> S
        where
            R: RangeBounds<usize>,
        {
            let (left, right) = self.get_bounds(rng);

            self.composite_range(left..right);

            let mut left_product = S::id();
            let mut right_product = S::id();

            let mut left = left + self.size;
            let mut right = right + self.size;

            while left < right {
                if left & 1 == 1 {
                    left_product = left_product.product(&self.data[left]);
                    left += 1;
                }

                if right & 1 == 1 {
                    right -= 1;
                    right_product = self.data[right].product(&right_product);
                }

                left >>= 1;
                right >>= 1;
            }

            left_product.product(&right_product)
        }

        pub fn product_all(&self) -> &S {
            &self.data[1]
        }

        pub fn apply_segment(&mut self, k: usize, f: &F) {
            self.data[k] = f.mapping(&self.data[k]);
            if k < self.size {
                self.lazy[k] = self.lazy[k].product(f);
            }
        }

        pub fn apply_range<R>(&mut self, rng: R, f: F)
        where
            R: RangeBounds<usize>,
        {
            let (mut left, mut right) = self.get_bounds(rng);

            self.composite_range(left..right);

            left += self.size;
            right += self.size;

            {
                let mut left = left;
                let mut right = right;

                while left < right {
                    if left & 1 == 1 {
                        self.apply_segment(left, &f);
                        left += 1;
                    }

                    if right & 1 == 1 {
                        right -= 1;
                        self.apply_segment(right, &f);
                    }

                    left >>= 1;
                    right >>= 1;
                }
            }

            for i in left.trailing_zeros() + 1..=self.log {
                self.up_product(left >> i);
            }

            for i in right.trailing_zeros() + 1..=self.log {
                self.up_product((right - 1) >> i);
            }
        }

        pub fn apply_all(&mut self, f: F) {
            self.apply_segment(1, &f);
        }
    }
}

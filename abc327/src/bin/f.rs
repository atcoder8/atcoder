use std::collections::VecDeque;

use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

fn main() {
    input! {
        (n, d, w): (usize, usize, usize),
        mut tx: [(usize, usize); n],
    }

    tx.sort_unstable();

    struct Mo {}

    impl Monoid for Mo {
        type S = i64;

        fn identity() -> Self::S {
            0
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }

    struct MMo;

    impl MapMonoid for MMo {
        type M = Mo;

        type F = i64;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            f + x
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }

    let mut lazy_segtree = LazySegtree::<MMo>::new(5e5 as usize);

    let mut ans = 0;
    let mut queue = VecDeque::new();
    for &(t, x) in &tx {
        queue.push_back((t, x));
        lazy_segtree.apply_range(x..x + w, 1);
        while let Some(&(head_t, head_x)) = queue.front() {
            if head_t + d <= t {
                lazy_segtree.apply_range(head_x..head_x + w, -1);
                queue.pop_front();
            } else {
                break;
            }
        }

        ans = ans.max(lazy_segtree.prod(..));
    }

    println!("{}", ans);
}

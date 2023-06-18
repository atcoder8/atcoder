use proconio::{input, marker::Usize1};

use crate::multiset::Multiset;

fn main() {
    input! {
        (n, k, q): (usize, usize, usize),
        xy: [(Usize1, usize); q],
    }

    let mut fa = 0;
    let mut aa = vec![0_usize; n];
    let mut large: Multiset<usize> = vec![0; k].into();
    let mut small: Multiset<usize> = vec![0; n - k].into();

    for &(x, y) in &xy {
        let elem = &mut aa[x];

        if small.remove(elem).is_none() {
            large.remove(elem);
            fa -= *elem;

            if let Some(&max) = small.max_element() {
                small.remove(&max);
                large.insert(max);
                fa += max;
            }
        }

        *elem = y;

        large.insert(*elem);
        fa += *elem;

        if large.len() > k {
            let min = *large.min_element().unwrap();
            large.remove(&min);
            small.insert(min);
            fa -= min;
        }

        println!("{}", fa);
    }
}

pub mod multiset {
    use std::{collections::BTreeMap, iter::FromIterator};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Multiset<T>
    where
        T: Ord,
    {
        map: BTreeMap<T, usize>,
        len: usize,
    }

    impl<T> Default for Multiset<T>
    where
        T: Ord,
    {
        fn default() -> Self {
            Self {
                map: Default::default(),
                len: Default::default(),
            }
        }
    }

    impl<T> FromIterator<T> for Multiset<T>
    where
        T: Ord,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut ms = Multiset::new();
            for value in iter {
                ms.insert(value);
            }

            ms
        }
    }

    impl<T> From<Vec<T>> for Multiset<T>
    where
        T: Ord,
    {
        fn from(value: Vec<T>) -> Self {
            value.into_iter().collect()
        }
    }

    impl<T> Multiset<T>
    where
        T: Ord,
    {
        pub fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                len: 0,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn len(&self) -> usize {
            self.len
        }

        /// Adds a `value` to the set.
        ///
        /// Returns the number of elements equal to `value` in the multiset.
        pub fn insert(&mut self, value: T) -> usize {
            let count = self.map.entry(value).or_default();
            *count += 1;
            self.len += 1;

            *count
        }

        /// If the multiset contains an element equal to `value`, removes it from the multiset.
        ///
        /// Returns the number of elements equal to `value` in the multiset after deletion.
        /// However, if the multiset does not contain any `value`, `None` is returned.
        pub fn remove(&mut self, value: &T) -> Option<usize> {
            if let Some(count) = self.map.get_mut(value) {
                *count -= 1;
                self.len -= 1;

                if *count == 0 {
                    self.map.remove(value);

                    return Some(0);
                }

                return Some(*count);
            }

            return None;
        }

        pub fn contains(&self, value: &T) -> bool {
            self.map.contains_key(value)
        }

        pub fn count(&self, value: &T) -> usize {
            *self.map.get(value).unwrap_or(&0)
        }

        pub fn min_element(&self) -> Option<&T> {
            if let Some((value, _)) = self.map.iter().next() {
                Some(value)
            } else {
                None
            }
        }

        pub fn max_element(&self) -> Option<&T> {
            if let Some((value, _)) = self.map.iter().next_back() {
                Some(value)
            } else {
                None
            }
        }

        pub fn clear(&mut self) {
            self.map.clear();
            self.len = 0;
        }

        pub fn remove_all(&mut self, value: &T) {
            self.len -= self.count(value);
            self.map.remove(&value);
        }
    }
}

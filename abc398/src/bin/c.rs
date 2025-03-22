use itertools::enumerate;
use proconio::input;
use unique_ordering::UniqueOrdering;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut uo = UniqueOrdering::<usize>::from(aa.clone());
    let mut group_by_number = vec![vec![]; n];
    for (i, &a) in enumerate(&aa) {
        group_by_number[uo.position(&a)].push(i);
    }

    let ans = group_by_number.iter().rev().find_map(|group| {
        if group.len() == 1 {
            Some(group[0])
        } else {
            None
        }
    });
    match ans {
        Some(ans) => println!("{}", ans + 1),
        None => println!("-1"),
    }
}

pub mod unique_ordering {
    //! Module for ordering unique elements.

    use std::ops::Index;

    /// Structure for ordering unique elements.
    #[derive(Debug, Clone)]
    pub struct UniqueOrdering<T> {
        /// A sequence containing the elements to be ordered.
        seq: Vec<T>,

        /// A flag indicating whether `seq` is sorted and deduplicated.
        organized: bool,
    }

    impl<T> Default for UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure for ordering unique elements.
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> From<Vec<T>> for UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure by initializing the elements to be ordered with `seq`.
        fn from(seq: Vec<T>) -> Self {
            Self {
                seq,
                organized: false,
            }
        }
    }

    impl<T> Index<usize> for UniqueOrdering<T> {
        type Output = T;

        /// Returns the `index`-th (0-based) unique element.
        fn index(&self, index: usize) -> &Self::Output {
            &self.seq[index]
        }
    }

    impl<T> UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure for ordering unique elements.
        pub fn new() -> Self {
            Self {
                seq: vec![],
                organized: true,
            }
        }

        /// Adds the elements to be ordered.
        pub fn push(&mut self, x: T) {
            self.seq.push(x);
            self.organized = false;
        }

        /// Appends all elements of the iterator to the elements to be ordered.
        pub fn extend<I>(&mut self, other: I)
        where
            I: IntoIterator<Item = T>,
        {
            self.seq.extend(other);
            self.organized = false;
        }

        /// Sorts the sequence of stored elements in ascending order and removes all duplicates.
        fn organize(&mut self) {
            if !self.organized {
                self.seq.sort_unstable();
                self.seq.dedup();
                self.organized = true;
            }
        }

        /// Returns the `x` position of the unique elements sorted in ascending order.
        pub fn position(&mut self, x: &T) -> usize {
            self.organize();

            self.seq.binary_search(x).unwrap_or_else(|_| {
                panic!("The position of `x` is undefined.");
            })
        }

        /// Returns the `index`-th (0-based) unique element.
        ///
        /// Returns `None` if the `index`-th element does not exist.
        pub fn get(&mut self, index: usize) -> Option<&T> {
            self.seq.get(index)
        }

        /// Returns the number of unique elements.
        pub fn unique_len(&mut self) -> usize {
            self.organize();

            self.seq.len()
        }
    }
}

use segtree::SegTree;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let cc = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>()
    };
    let xx = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>()
    };

    let mut ans = 0;

    let mut st = SegTree::<usize>::new(n);
    for &x in xx.iter() {
        ans += st.prod(x + 1, n);
        st.set(x, st.get(x) + 1);
    }

    let cn = cc.iter().max().unwrap() + 1;

    let mut cc2 = vec![vec![]; cn];
    for (&c, &x) in cc.iter().zip(xx.iter()) {
        cc2[c].push(x);
    }

    for c2 in cc2.iter() {
        let (zip, unzip) = coordinate_compression(c2);

        let mut st = SegTree::<usize>::new(unzip.len());

        for &z in zip.iter() {
            ans -= st.prod(z + 1, unzip.len());
            st.set(z, st.get(z) + 1);
        }
    }

    println!("{}", ans);
}

fn coordinate_compression<T>(vec: &Vec<T>) -> (Vec<usize>, Vec<T>)
where
    T: Clone + Ord,
{
    let mut unzip = vec.clone();
    unzip.sort_unstable();
    unzip.dedup();
    let mut zip = vec![0; vec.len()];
    for i in 0..vec.len() {
        zip[i] = unzip.binary_search(&vec[i]).unwrap() as usize;
    }
    (zip, unzip)
}

impl segtree::Monoid for usize {
    type S = usize;

    fn e() -> Self::S {
        0
    }

    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}

pub mod segtree {
    pub fn ceil_log2(n: usize) -> usize {
        let mut ret = 0;
        while (1 << ret) < n {
            ret += 1;
        }
        ret
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

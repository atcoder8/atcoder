use std::{cmp::Reverse, ops};

use proconio::{input, marker::Usize1};
use union_find::*;

const MAX_K: usize = 10;

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut uf = UnionFindWithCMonoid::from((0..n).map(|i| TopIndexes::new(i)));
    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (u, v): (Usize1, Usize1),
            }

            uf.merge(u, v);
        } else {
            input! {
                (v, k): (Usize1, usize),
            }

            match uf.value(v).nth(k - 1) {
                Some(idx) => println!("{}", idx + 1),
                None => println!("-1"),
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct TopIndexes(Vec<usize>);

impl TopIndexes {
    fn new(idx: usize) -> Self {
        Self(vec![idx])
    }

    fn append(&mut self, other: &mut TopIndexes) {
        self.0.append(&mut other.0);
        self.0.sort_unstable_by_key(|&idx| Reverse(idx));
        self.0.truncate(MAX_K);
    }

    fn nth(&self, k: usize) -> Option<usize> {
        self.0.get(k).cloned()
    }
}

impl ops::AddAssign<Self> for TopIndexes {
    fn add_assign(&mut self, rhs: Self) {
        let mut rhs = rhs;
        self.append(&mut rhs);
    }
}

pub mod union_find {
    pub use commutative_monoid::*;

    /// 素集合データ構造に共通する操作です。
    /// 連結成分の併合にランクではなく要素数を用います。
    /// 代表元の探索の際に経路圧縮を行うことを想定して、このトレイトが定義する全てのメソッドは呼び出し元オブジェクトへの可変参照をとります。
    pub trait UnionBySizeMut {
        /// 頂点`u`が所属する連結成分の代表元を返します。
        fn root(&mut self, u: usize) -> usize;

        /// 2つの頂点`u, v`について、それぞれが所属する連結成分を併合します。
        ///
        /// 異なる連結成分が新しく併合された場合のみ、`true`を返します。
        fn merge(&mut self, u: usize, v: usize) -> bool;

        /// 頂点`u`が所属している連結成分の要素数を返します。
        fn size(&mut self, u: usize) -> usize;

        /// 2つの頂点`u, v`が同じ連結成分に所属しているかどうかを判定します。
        fn same(&mut self, u: usize, v: usize) -> bool {
            self.root(u) == self.root(v)
        }
    }

    pub mod commutative_monoid {
        use std::{
            mem,
            ops::{self, AddAssign},
        };

        use super::UnionBySizeMut;

        /// 連結成分の代表元が保持する要約です。
        #[derive(Debug, Clone)]
        struct Summary<CMonoid>
        where
            CMonoid: Clone + ops::AddAssign<CMonoid>,
        {
            /// 連結成分の要素数
            size: usize,

            /// 連結成分に対応付けられた可換モノイドの元
            value: CMonoid,
        }

        /// 連結成分の併合と可換モノイドの演算を対応付けたUnionFindです。
        /// 連結成分ごとに可換モノイドの元を保持し、2つの連結成分が併合される際に新たな連結成分と演算結果が対応付けられます。
        #[derive(Debug, Clone)]
        pub struct UnionFindWithCMonoid<CMonoid>
        where
            CMonoid: Clone + ops::AddAssign<CMonoid>,
        {
            /// parents\[i\]: 頂点iの親 (代表元の場合は自分自身)
            parents: Vec<usize>,

            /// 代表元に対し、連結成分の大きさと可換モノイドの元が保持されます。
            table: Vec<Option<Summary<CMonoid>>>,

            /// 連結成分の個数
            num_groups: usize,
        }

        impl<CMonoid> UnionBySizeMut for UnionFindWithCMonoid<CMonoid>
        where
            CMonoid: Clone + ops::AddAssign<CMonoid>,
        {
            fn root(&mut self, u: usize) -> usize {
                let mut path = vec![];
                let mut current = u;
                while self.parents[current] != current {
                    path.push(current);
                    current = self.parents[current];
                }

                path.iter().for_each(|&node| self.parents[node] = current);

                current
            }

            fn merge(&mut self, u: usize, v: usize) -> bool {
                let mut root_u = self.root(u);
                let mut root_v = self.root(v);

                if root_u == root_v {
                    return false;
                }

                if self.size(root_u) < self.size(root_v) {
                    mem::swap(&mut root_u, &mut root_v);
                }

                let summary_v = mem::take(&mut self.table[root_v]).unwrap();
                let summary_u = self.table[root_u].as_mut().unwrap();

                self.parents[root_v] = root_u;

                summary_u.size += summary_v.size;
                summary_u.value += summary_v.value;

                self.num_groups -= 1;

                true
            }

            fn size(&mut self, u: usize) -> usize {
                let root = self.root(u);
                self.table[root].as_ref().unwrap().size
            }
        }

        impl<CMonoid, I> From<I> for UnionFindWithCMonoid<CMonoid>
        where
            CMonoid: Clone + ops::AddAssign<CMonoid>,
            I: IntoIterator<Item = CMonoid>,
        {
            /// モノイドの元のイテレータまたはイテラブルなコレクションからUnionFindオブジェクトを生成します。
            fn from(iterable: I) -> Self {
                let table = iterable
                    .into_iter()
                    .map(|value| Some(Summary { size: 1, value }))
                    .collect::<Vec<_>>();
                let num_elements = table.len();

                Self {
                    parents: (0..num_elements).collect(),
                    table,
                    num_groups: num_elements,
                }
            }
        }

        impl<CMonoid> UnionFindWithCMonoid<CMonoid>
        where
            CMonoid: Clone + AddAssign<CMonoid>,
        {
            /// グラフ全体の頂点の個数を返します。
            pub fn num_elements(&self) -> usize {
                self.parents.len()
            }

            /// 連結成分の個数を返します。
            pub fn num_groups(&self) -> usize {
                self.num_groups
            }

            /// 指定された頂点が所属する連結成分に紐付けられた可換モノイドの元を返します。
            pub fn value(&mut self, u: usize) -> &CMonoid {
                let root = self.root(u);
                &self.table[root].as_ref().unwrap().value
            }

            /// グラフ全体の頂点を連結成分ごとに分割します。
            /// 各連結成分の頂点のインデックスは昇順に並んでおり、連結成分はリストの先頭のインデックスについて昇順に並んでいます。
            pub fn groups(&mut self) -> Vec<Vec<usize>> {
                let element_num = self.table.len();

                let mut groups: Vec<Vec<usize>> = vec![];
                let mut leader_to_group: Vec<Option<usize>> = vec![None; self.table.len()];
                for u in 0..element_num {
                    let root_u = self.root(u);
                    if let Some(group_idx) = leader_to_group[root_u] {
                        groups[group_idx].push(u);
                    } else {
                        leader_to_group[root_u] = Some(groups.len());
                        groups.push(vec![u]);
                    }
                }

                groups
            }
        }
    }
}

use itertools::{enumerate, Itertools};
use proconio::input;

use crate::min_heap::MinHeap;

fn main() {
    input! {
        (n, k): (usize, usize),
        abc: [(usize, usize, usize); n],
    }

    let entry_events = enumerate(&abc).map(|(i, &(a, _b, c))| Event::new(i, a, c));
    let mut heap: MinHeap<Event, usize> = MinHeap::new(|event| event.time);

    let mut entry_times = vec![0_usize; n];
    let mut num_visitors = 0_usize;
    let mut current_time = 0_usize;
    for entry_event in entry_events {
        while num_visitors + entry_event.count > k {
            let leave_event = heap.pop().unwrap();
            current_time = current_time.max(leave_event.time);
            num_visitors -= leave_event.count;
        }

        current_time = current_time.max(entry_event.time);
        entry_times[entry_event.id] = current_time;
        num_visitors += entry_event.count;
        heap.push(Event::new(
            entry_event.id,
            current_time + abc[entry_event.id].1,
            entry_event.count,
        ));
    }

    println!("{}", entry_times.iter().join("\n"));
}

#[derive(Debug, Clone, Copy)]
struct Event {
    id: usize,
    time: usize,
    count: usize,
}

impl Event {
    fn new(id: usize, time: usize, count: usize) -> Self {
        Event { id, time, count }
    }
}

pub mod min_heap {
    use std::{cmp::Reverse, collections::BinaryHeap, iter::FusedIterator, marker::PhantomData};

    /// キー抽出関数を用いて作成されたヒープノード
    #[derive(Debug, Clone)]
    struct HeapNode<Value, SortKey: Ord> {
        value: Value,
        sort_key: SortKey,
    }

    impl<Value, SortKey: Ord> PartialEq for HeapNode<Value, SortKey> {
        fn eq(&self, other: &Self) -> bool {
            self.sort_key == other.sort_key
        }
    }

    impl<Value, SortKey: Ord> Eq for HeapNode<Value, SortKey> where SortKey: Ord {}

    impl<Value, SortKey: Ord> PartialOrd for HeapNode<Value, SortKey> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<Value, SortKey: Ord> Ord for HeapNode<Value, SortKey> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.sort_key.cmp(&other.sort_key)
        }
    }

    /// キー抽出関数を用いてヒープノードを作成するBuilderです。
    #[derive(Debug, Clone)]
    struct NodeBuilder<Value, SortKey: Ord> {
        value: PhantomData<Value>,
        sort_key: PhantomData<SortKey>,
        extract_key: fn(&Value) -> SortKey,
    }

    impl<Value, SortKey: Ord> NodeBuilder<Value, SortKey> {
        fn new(extract_key: fn(&Value) -> SortKey) -> Self {
            Self {
                value: PhantomData,
                sort_key: PhantomData,
                extract_key,
            }
        }

        fn build(&self, value: Value) -> HeapNode<Value, SortKey> {
            let sort_key = (self.extract_key)(&value);
            HeapNode { value, sort_key }
        }
    }

    /// キー抽出関数を保持する最小ヒープです。
    #[derive(Debug, Clone)]
    pub struct MinHeap<Value, SortKey: Ord> {
        /// キー抽出関数を用いて順序付けされたヒープ
        heap: BinaryHeap<Reverse<HeapNode<Value, SortKey>>>,

        /// キー抽出関数を用いてヒープノードを作成するBuilder
        node_builder: NodeBuilder<Value, SortKey>,
    }

    impl<Value, SortKey: Ord> Extend<Value> for MinHeap<Value, SortKey> {
        fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
            let nodes = iter.into_iter().map(|value| self.node_builder.build(value));
            self.heap.extend(nodes.map(Reverse));
        }
    }

    impl<Value, SortKey: Ord> MinHeap<Value, SortKey> {
        /// 空のヒープを作成します。ヒープ内部の順序付けはキー抽出関数を用いて行われます。
        pub fn new(extract_key: fn(&Value) -> SortKey) -> Self {
            Self {
                heap: BinaryHeap::new(),
                node_builder: NodeBuilder::new(extract_key),
            }
        }

        /// 単一の値を持つヒープを生成します。
        pub fn from_value(value: Value, extract_key: fn(&Value) -> SortKey) -> Self {
            let mut min_heap = Self::new(extract_key);
            min_heap.push(value);

            min_heap
        }

        /// 値のコレクションをヒープに変換します。
        pub fn from_iter<I>(values: I, extract_key: fn(&Value) -> SortKey) -> Self
        where
            I: IntoIterator<Item = Value>,
        {
            let mut min_heap = Self::new(extract_key);
            min_heap.extend(values);

            min_heap
        }

        /// ヒープの要素数を返します。
        pub fn len(&self) -> usize {
            self.heap.len()
        }

        /// ヒープが空であるかどうかを表すブール値を返します。
        pub fn is_empty(&self) -> bool {
            self.heap.is_empty()
        }

        /// ヒープの全ての要素を訪れるイテレータです。
        /// 要素の訪問順は不定です。
        pub fn iter(&self) -> impl Iterator<Item = &Value> {
            self.heap.iter().map(|Reverse(node)| &node.value)
        }

        /// ヒープ内の値のうち、キー抽出関数を最小化するものを返します。
        /// ヒープが空である場合は`None`を返します。
        pub fn peek(&self) -> Option<&Value> {
            Some(&self.heap.peek()?.0.value)
        }

        /// ヒープに値を追加します。
        pub fn push(&mut self, value: Value) {
            self.heap.push(Reverse(self.node_builder.build(value)));
        }

        /// ヒープ内の値のうち、キー抽出関数を最小化するものを削除して返します。
        /// ヒープが空である場合は`None`を返します。
        pub fn pop(&mut self) -> Option<Value> {
            Some(self.heap.pop()?.0.value)
        }

        /// ヒープ内の全ての要素を作成します。
        pub fn clear(&mut self) {
            self.heap.clear();
        }

        /// ヒープに格納された値のうち、`predicate`により`true`と判定されるもののみを残します。
        pub fn retain<Predicate>(&mut self, mut predicate: Predicate)
        where
            Predicate: FnMut(&Value) -> bool,
        {
            self.heap.retain(|Reverse(node)| predicate(&node.value));
        }

        /// 追加の要素のための領域を確保します。
        /// `reserve`を呼び出した後、ヒープの容量は`self.len() + additional`以上になります。
        /// 頻繁なメモリの再確保を避けるために領域を余分に確保する可能性があります。
        pub fn reserve(&mut self, additional: usize) {
            self.heap.reserve(additional);
        }

        /// 追加の要素のための領域を確保します。
        /// `reserve_exact`を呼び出した後、ヒープの容量は`self.len() + additional`以上になります。
        /// `reserve`と異なり、頻繁なメモリの再確保を避けるための余分な領域の確保を行いません。
        pub fn reserve_exact(&mut self, additional: usize) {
            self.heap.reserve_exact(additional);
        }

        /// ヒープの余分な容量を破棄します。
        pub fn shrink_to_fit(&mut self) {
            self.heap.shrink_to_fit();
        }

        /// `min_capacity`を下限としてヒープの余分な容量を破棄します。
        pub fn shrink_to(&mut self, min_capacity: usize) {
            self.heap.shrink_to(min_capacity);
        }
    }

    /// ヒープの要素をキー抽出関数の戻り値に関して昇順に返すイテレータです。
    #[derive(Debug, Clone)]
    pub struct MinHeapIter<Value, SortKey: Ord> {
        min_heap: MinHeap<Value, SortKey>,
    }

    impl<Value, SortKey: Ord> ExactSizeIterator for MinHeapIter<Value, SortKey> {
        fn len(&self) -> usize {
            self.min_heap.len()
        }
    }

    impl<Value, SortKey: Ord> FusedIterator for MinHeapIter<Value, SortKey> {}

    impl<Value, SortKey: Ord> Iterator for MinHeapIter<Value, SortKey> {
        type Item = Value;

        fn next(&mut self) -> Option<Self::Item> {
            self.min_heap.pop()
        }
    }

    impl<Value, SortKey: Ord> IntoIterator for MinHeap<Value, SortKey> {
        type Item = Value;

        type IntoIter = MinHeapIter<Value, SortKey>;

        /// ヒープを消費してイテレータを生成します。
        /// 生成されたイテレータはヒープ内の全ての値をキー抽出関数の戻り値に関して昇順に返します。
        fn into_iter(self) -> Self::IntoIter {
            MinHeapIter { min_heap: self }
        }
    }
}

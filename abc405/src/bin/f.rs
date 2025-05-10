// unfinished

use fixedbitset::FixedBitSet;
use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (_n, m): (usize, usize),
        ab: [(Usize1, usize); m],
        q: usize,
        cd: [(Usize1, Usize1); q],
    }

    let events = enumerate(&ab)
        .flat_map(|(i, &(a, b))| {
            [
                Event::ChordIn { id: i, point: a },
                Event::ChordOut { id: i, point: b },
            ]
        })
        .chain(enumerate(&cd).flat_map(|(i, &(c, d))| {
            [
                Event::QueryIn { id: i, point: c },
                Event::QueryOut { id: i, point: d },
            ]
        }))
        .sorted_unstable_by_key(|event| event.to_time());

    let mut bits = FixedBitSet::with_capacity(m);

    let mut from = vec![None::<FixedBitSet>; q];
    let mut to = vec![None::<FixedBitSet>; q];
    for event in events {
        match event {
            Event::ChordIn { id, point: _ } => bits.insert(id),
            Event::ChordOut { id, point: _ } => bits.set(id, false),
            Event::QueryIn { id, point: _ } => from[id] = Some(bits.clone()),
            Event::QueryOut { id, point: _ } => to[id] = Some(bits.clone()),
        }
    }

    let ans = izip!(from, to)
        .map(|(from, to)| {
            let mut from = from.unwrap();
            from.symmetric_difference_with(&to.unwrap());
            from.count_ones(..)
        })
        .join("\n");
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy)]
enum Event {
    ChordIn { id: usize, point: usize },
    ChordOut { id: usize, point: usize },
    QueryIn { id: usize, point: usize },
    QueryOut { id: usize, point: usize },
}

impl Event {
    fn to_time(&self) -> usize {
        match *self {
            Event::ChordIn { id: _, point } => point,
            Event::ChordOut { id: _, point } => point,
            Event::QueryIn { id: _, point } => point,
            Event::QueryOut { id: _, point } => point,
        }
    }
}

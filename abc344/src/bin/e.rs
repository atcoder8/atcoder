use std::collections::BTreeMap;

use itertools::{chain, Itertools};
use proconio::input;

type List = BTreeMap<usize, (usize, usize)>;

const HEAD: usize = 0;
const TAIL: usize = 10_usize.pow(9) + 1;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        q: usize,
    }

    // map[key]: (prev, next)
    // prev(HEAD) = HEAD
    // next(TAIL) = TAIL
    let mut list = List::from([(HEAD, (HEAD, TAIL)), (TAIL, (HEAD, TAIL))]);
    for (&x, &y) in chain!(&[HEAD], &aa).tuple_windows() {
        insert(&mut list, x, y);
    }

    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (x, y): (usize, usize),
            }

            insert(&mut list, x, y);
        } else {
            input! {
                x: usize,
            }

            remove(&mut list, x);
        }
    }

    show(&list);
}

fn insert(list: &mut List, x: usize, y: usize) {
    let (_prev, next) = list[&x];
    list.insert(y, (x, next));
    list.get_mut(&x).unwrap().1 = y;
    list.get_mut(&next).unwrap().0 = y;
}

fn remove(list: &mut List, x: usize) {
    let (prev, next) = list[&x];
    list.remove(&x);
    list.get_mut(&prev).unwrap().1 = next;
    list.get_mut(&next).unwrap().0 = prev;
}

fn show(list: &List) {
    let mut ans = vec![];

    let mut cur = list[&HEAD].1;
    while cur != TAIL {
        ans.push(cur);
        cur = list[&cur].1;
    }

    println!("{}", ans.iter().join(" "));
}

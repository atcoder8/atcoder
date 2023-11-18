use im_rc::HashSet;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        cc: [usize; n],
        ab: [(Usize1, Usize1); q],
    }

    let mut boxes = cc.iter().map(|&c| HashSet::from(vec![c])).collect_vec();
    for &(a, b) in &ab {
        if boxes[a].len() > boxes[b].len() {
            boxes.swap(a, b);
        }

        let balls = std::mem::replace(&mut boxes[a], HashSet::new());
        boxes[b].extend(balls);

        println!("{}", boxes[b].len());
    }
}

use im_rc::HashMap;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (_w, _h): (usize, usize),
        n: usize,
        pq: [(usize, usize); n],
        na: usize,
        aa: [usize; na],
        nb: usize,
        bb: [usize; nb],
    }

    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    for &(p, q) in &pq {
        let x_pos = aa.upper_bound(&p);
        let y_pos = bb.upper_bound(&q);

        *map.entry((x_pos, y_pos)).or_insert(0) += 1;
    }

    let min = if map.len() == (na + 1) * (nb + 1) {
        *map.values().min().unwrap()
    } else {
        0
    };
    let max = *map.values().max().unwrap();

    println!("{} {}", min, max);
}

use itertools::Itertools;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 8] = [
    (!0, !0),
    (!0, 0),
    (!0, 1),
    (0, !0),
    (0, 1),
    (1, !0),
    (1, 0),
    (1, 1),
];

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let snuke = "snuke".chars().collect_vec();
    let len = snuke.len();

    let is_match = |start_coord: (usize, usize), diff: (usize, usize)| {
        let (mut i, mut j) = start_coord;
        let (dx, dy) = diff;

        for k in 0..len {
            if !(i < h && j < w) || ss[i][j] != snuke[k] {
                return false;
            }

            i = i.wrapping_add(dx);
            j = j.wrapping_add(dy);
        }

        true
    };

    for si in 0..h {
        for sj in 0..w {
            for &(dx, dy) in &DIFFS {
                if is_match((si, sj), (dx, dy)) {
                    let (mut i, mut j) = (si, sj);

                    for _ in 0..len {
                        println!("{} {}", i + 1, j + 1);
                        i = i.wrapping_add(dx);
                        j = j.wrapping_add(dy);
                    }
                }
            }
        }
    }
}

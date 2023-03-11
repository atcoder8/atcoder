use im_rc::HashSet;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [[usize; w]; h],
    }

    let seq = (0..(h + w - 2)).collect_vec();

    let mut ans = 0;

    for comb in seq.iter().combinations(h - 1) {
        let mut v_move = vec![false; h + w - 2];
        for &x in comb {
            v_move[x] = true;
        }

        let mut used = HashSet::new();
        used.insert(aaa[0][0]);

        let mut x = 0;
        let mut y = 0;

        let mut flag = true;

        for i in 0..(h + w - 2) {
            if v_move[i] {
                x += 1;
            } else {
                y += 1;
            }

            if used.contains(&aaa[x][y]) {
                flag = false;
                break;
            }

            used.insert(aaa[x][y]);
        }

        if flag {
            ans += 1;
        }
    }

    println!("{}", ans);
}

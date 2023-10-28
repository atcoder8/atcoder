use itertools::Itertools;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("Yes\n{}", ans),
        None => println!("No"),
    }
}

fn solve() -> Option<String> {
    input! {
        (n, x, y): (usize, i64, i64),
        aa: [i64; n],
    }

    let ay = aa.iter().step_by(2).cloned().collect_vec();
    let y_bit = match search_bit_pair(&ay, y) {
        Some((bit1, bit2)) => bit1 | bit2 << ay.len() / 2,
        None => return None,
    };

    let ax = aa[1..].iter().step_by(2).cloned().collect_vec();
    let x_bit = match search_bit_pair(&ax, x) {
        Some((bit1, bit2)) => bit1 | bit2 << ax.len() / 2,
        None => return None,
    };

    let mut rotations = String::new();
    let mut prev = true;
    for i in 0..n {
        if i % 2 == 0 {
            let cur = y_bit >> i / 2 & 1 == 0;
            let rotation = match (prev, cur) {
                (true, true) => 'L',
                (true, false) => 'R',
                (false, true) => 'R',
                (false, false) => 'L',
            };

            rotations.push(rotation);
            prev = cur;
        } else {
            let cur = x_bit >> i / 2 & 1 == 0;
            let rotation = match (prev, cur) {
                (true, true) => 'R',
                (true, false) => 'L',
                (false, true) => 'L',
                (false, false) => 'R',
            };

            rotations.push(rotation);
            prev = cur;
        }
    }

    Some(rotations)
}

fn calc_bit_dists(aa: &[i64]) -> Vec<(usize, i64)> {
    let mut dists = vec![(0, 0); 1 << aa.len()];
    dists[0] = (0, aa.iter().sum());
    for bit in 1_usize..1 << aa.len() {
        let lsb = bit.trailing_zeros() as usize;
        dists[bit] = (bit, dists[bit ^ 1 << lsb].1 - 2 * aa[lsb]);
    }
    dists.sort_unstable_by_key(|x| x.1);

    dists
}

fn search_bit_pair(dists: &[i64], dest: i64) -> Option<(usize, usize)> {
    let dists1 = calc_bit_dists(&dists[..dists.len() / 2]);
    let dists2 = calc_bit_dists(&dists[dists.len() / 2..]);

    for (bit1, dist1) in dists1 {
        if let Ok(pos) = dists2.binary_search_by_key(&(dest - dist1), |&(_, dist2)| dist2) {
            return Some((bit1, dists2[pos].0));
        }
    }

    None
}

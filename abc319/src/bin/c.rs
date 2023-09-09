use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        ccc: [[usize; 3]; 3],
    }

    let check = |known: &Vec<Vec<bool>>| {
        for i in 0..3 {
            let seq = (0..3)
                .filter(|&j| known[i][j])
                .map(|j| ccc[i][j])
                .collect_vec();
            if seq.len() == 2 && seq.iter().all_equal() {
                return false;
            }
        }

        for j in 0..3 {
            let seq = (0..3)
                .filter(|&i| known[i][j])
                .map(|i| ccc[i][j])
                .collect_vec();
            if seq.len() == 2 && seq.iter().all_equal() {
                return false;
            }
        }

        let seq = (0..3)
            .filter(|&i| known[i][i])
            .map(|i| ccc[i][i])
            .collect_vec();
        if seq.len() == 2 && seq.iter().all_equal() {
            return false;
        }

        let seq = (0..3)
            .filter(|&i| known[i][2 - i])
            .map(|i| ccc[i][2 - i])
            .collect_vec();
        if seq.len() == 2 && seq.iter().all_equal() {
            return false;
        }

        true
    };

    let mut cnt = 0;
    for orders in (0..9).permutations(9) {
        let mut flag = true;
        let mut known = vec![vec![false; 3]; 3];
        for i in 0..9 {
            let order = orders[i];
            known[order / 3][order % 3] = true;

            if !check(&known) {
                flag = false;
                break;
            }
        }

        cnt += flag as usize;
    }

    println!("{}", cnt as f64 / 362880.0);
}

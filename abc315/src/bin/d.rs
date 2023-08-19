use proconio::{input, marker::Bytes};

fn main() {
    input! {
        (h, w): (usize, usize),
        ccc: [Bytes; h],
    }

    let ccc: Vec<Vec<usize>> = ccc
        .into_iter()
        .map(|cc| cc.into_iter().map(|c| (c - b'a') as usize).collect())
        .collect();

    let mut colors_per_hor = vec![vec![0; 26]; h];
    let mut colors_per_ver = vec![vec![0; 26]; w];
    for i in 0..h {
        for j in 0..w {
            colors_per_hor[i][ccc[i][j]] += 1;
            colors_per_ver[j][ccc[i][j]] += 1;
        }
    }

    let mut removed = vec![vec![false; w]; h];

    loop {
        let mut remove_vec = vec![];

        for i in 0..h {
            let sum: usize = colors_per_hor[i].iter().sum();
            if sum >= 2 && colors_per_hor[i].iter().any(|&cnt| cnt == sum) {
                for j in 0..w {
                    if removed[i][j] {
                        continue;
                    }

                    remove_vec.push((i, j));
                }
            }
        }

        for j in 0..w {
            let sum: usize = colors_per_ver[j].iter().sum();
            if sum >= 2 && colors_per_ver[j].iter().any(|&cnt| cnt == sum) {
                for i in 0..h {
                    if removed[i][j] {
                        continue;
                    }

                    remove_vec.push((i, j));
                }
            }
        }

        remove_vec.sort_unstable();
        remove_vec.dedup();

        for &(i, j) in &remove_vec {
            let c = ccc[i][j];
            colors_per_hor[i][c] -= 1;
            colors_per_ver[j][c] -= 1;
            removed[i][j] = true;
        }

        if remove_vec.is_empty() {
            break;
        }
    }

    let mut ans = 0;
    for row in 0..h {
        for col in 0..w {
            ans += !removed[row][col] as usize;
        }
    }
    println!("{}", ans);
}

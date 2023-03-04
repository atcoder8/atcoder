use im_rc::HashSet;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, _w, n): (usize, usize, usize),
        xy: [(Usize1, Usize1); n],
    }

    let mut horizontal_black = vec![vec![]; h];

    for &(x, y) in &xy {
        horizontal_black[x].push(y);
    }

    horizontal_black.iter_mut().for_each(|x| x.sort_unstable());

    let mut ans = 0;

    let mut black_set: HashSet<usize> = HashSet::new();

    for i in (0..h).rev() {
        let next_black_set: HashSet<usize> = horizontal_black[i].iter().cloned().collect();

        let mut diff: Vec<usize> = black_set
            .difference(next_black_set.clone())
            .iter()
            .cloned()
            .collect();
        diff.sort_unstable();

        if !diff.is_empty() {
            ans += 2
                * (0..(diff.len() - 1))
                    .filter(|&i| diff[i + 1] != diff[i] + 1)
                    .count()
                + 2;
            if diff[0] == 0 {
                ans -= 1;
            }
        }

        black_set = next_black_set;
    }

    println!("{}", ans);
}

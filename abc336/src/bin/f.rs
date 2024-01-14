use im_rc::hashmap;
use itertools::{iproduct, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [[Usize1; w]; h],
    }

    let create_rotated_grids = |grid: &[Vec<usize>]| {
        let mut rotated_grids = vec![];

        for (top, left) in iproduct!(0..=1, 0..=1) {
            let mut rotated_grid = grid.to_owned();
            for (row, col) in iproduct!(0..h - 1, 0..w - 1) {
                rotated_grid[row + top][col + left] = grid[h - 2 - row + top][w - 2 - col + left];
            }

            rotated_grids.push(rotated_grid);
        }

        rotated_grids
    };

    let create_half_grids = |start: &[Vec<usize>]| {
        let mut grids = hashmap! {start.to_owned() => 0_usize};
        for cnt in 1..=10 {
            let mut rotated_grids = vec![];
            for (goal, _) in &grids {
                rotated_grids.extend(create_rotated_grids(&goal));
            }

            for rotated_grid in rotated_grids {
                if !grids.contains_key(&rotated_grid) {
                    grids.insert(rotated_grid, cnt);
                }
            }
        }

        grids
    };

    let aligned_grid = (0..h)
        .map(|row| (0..w).map(|col| row * w + col).collect_vec())
        .collect_vec();
    let half_grids_1 = create_half_grids(&aligned_grid);
    let half_grids_2 = create_half_grids(&sss);

    let mut ans = None;
    for (grid1, cnt1) in half_grids_1 {
        if let Some(cnt2) = half_grids_2.get(&grid1) {
            update_cost(&mut ans, cnt1 + cnt2);
        }
    }

    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

/// Updates the minimum cost.
/// If `cost` is `None`, always updated to the candidate cost.
///
/// # Arguments
///
/// * `cost` - Reference variable for the cost to be updated.
/// * `cand_cost` - Candidate cost to update.
pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: PartialOrd,
{
    if cost.as_ref().is_some_and(|cost| cost <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}

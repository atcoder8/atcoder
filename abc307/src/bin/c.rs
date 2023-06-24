use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (ha, _wa): (usize, usize),
        aa: [Chars; ha],
        (hb, _wb): (usize, usize),
        bb: [Chars; hb],
        (hx, _wx): (usize, usize),
        xx: [Chars; hx],
    }

    let find_positions = |sheet: &Vec<Vec<char>>| {
        let mut positions = vec![];
        for i in 0..sheet.len() {
            for j in 0..sheet[0].len() {
                if sheet[i][j] == '#' {
                    positions.push((i, j));
                }
            }
        }

        positions
    };

    let a_pos = find_positions(&aa);
    let b_pos = find_positions(&bb);
    let x_pos = find_positions(&xx);

    let mut x_top = 100;
    let mut x_left = 100;
    for &(x, y) in &x_pos {
        x_top = x_top.min(x);
        x_left = x_left.min(y);
    }

    let shifted_x_pos = x_pos
        .iter()
        .map(|&(x, y)| (x - x_top, y - x_left))
        .sorted()
        .collect_vec();

    for top_a in 0..30 {
        for left_a in 0..30 {
            let shifted_a_pos = a_pos
                .iter()
                .map(|&(x, y)| (x + top_a, y + left_a))
                .collect_vec();

            for top_b in 0..30 {
                for left_b in 0..30 {
                    let shifted_b_pos = b_pos
                        .iter()
                        .map(|&(x, y)| (x + top_b, y + left_b))
                        .collect_vec();

                    let ab_pos = shifted_a_pos
                        .iter()
                        .chain(&shifted_b_pos)
                        .cloned()
                        .sorted()
                        .dedup()
                        .collect_vec();

                    let mut ab_top = 100;
                    let mut ab_left = 100;
                    for &(x, y) in &ab_pos {
                        ab_top = ab_top.min(x);
                        ab_left = ab_left.min(y);
                    }

                    let shifted_ab_pos = ab_pos
                        .iter()
                        .map(|&(x, y)| (x - ab_top, y - ab_left))
                        .sorted()
                        .collect_vec();

                    if shifted_x_pos == shifted_ab_pos {
                        println!("Yes");
                        std::process::exit(0);
                    }
                }
            }
        }
    }

    println!("No");
}

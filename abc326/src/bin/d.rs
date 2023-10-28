use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    let ans = solve();
    match ans {
        Some(board) => {
            println!(
                "Yes\n{}",
                board
                    .iter()
                    .map(|line| line.iter().collect::<String>())
                    .join("\n")
            );
        }
        None => println!("No"),
    }
}

fn solve() -> Option<Vec<Vec<char>>> {
    input! {
        n: usize,
        rr: Chars,
        cc: Chars,
    }

    let is_match = |board: &[Vec<char>]| {
        for i in 0..n {
            if (0..n)
                .map(|j| board[i][j])
                .filter(|&c| c != '.')
                .sorted_unstable()
                .collect_vec()
                != vec!['A', 'B', 'C']
            {
                return false;
            }
        }

        for j in 0..n {
            if (0..n)
                .map(|i| board[i][j])
                .filter(|&c| c != '.')
                .sorted_unstable()
                .collect_vec()
                != vec!['A', 'B', 'C']
            {
                return false;
            }
        }

        for (i, &r) in rr.iter().enumerate() {
            if (0..n).map(|j| board[i][j]).find(|&c| c != '.').unwrap() != r {
                return false;
            }
        }

        for (j, &c) in cc.iter().enumerate() {
            if (0..n).map(|i| board[i][j]).find(|&c| c != '.').unwrap() != c {
                return false;
            }
        }

        true
    };

    let is_ok = |board: &[Vec<char>]| {
        for j in 0..n {
            if !(0..board.len())
                .map(|i| board[i][j])
                .filter(|&c| c != '.')
                .all_unique()
            {
                return false;
            }
        }

        true
    };

    let mut stack: Vec<Vec<Vec<char>>> = vec![vec![]];
    while let Some(board) = stack.pop() {
        if board.len() == n {
            if is_match(&board) {
                return Some(board);
            }

            continue;
        }

        for positions in (0..n).permutations(3) {
            let mut line = vec!['.'; n];
            for (c_idx, &pos) in positions.iter().enumerate() {
                line[pos] = ['A', 'B', 'C'][c_idx];
            }

            let mut next_board = board.clone();
            next_board.push(line);

            if is_ok(&next_board) {
                stack.push(next_board);
            }
        }
    }

    None
}

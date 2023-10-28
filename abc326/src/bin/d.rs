use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        rr: Chars,
        cc: Chars,
    }

    match solve(&rr, &cc) {
        Some(board) => {
            println!(
                "Yes\n{}",
                board
                    .chunks(n)
                    .map(|line| line.iter().collect::<String>())
                    .join("\n")
            );
        }
        None => println!("No"),
    }
}

fn solve(rr: &[char], cc: &[char]) -> Option<Vec<char>> {
    let n = rr.len();

    let is_ok = |board: &[char]| {
        for col in 0..n {
            if let Some(top) = (col..board.len())
                .step_by(n)
                .map(|idx| board[idx])
                .find(|&c| c != '.')
            {
                if top != cc[col] {
                    return false;
                }
            }

            if !(col..board.len())
                .step_by(n)
                .map(|idx| board[idx])
                .filter(|&c| c != '.')
                .all_unique()
            {
                return false;
            }
        }

        true
    };

    let mut stack: Vec<Vec<char>> = vec![vec![]];
    while let Some(board) = stack.pop() {
        if board.len() == n.pow(2) {
            return Some(board);
        }

        let r = rr[board.len() / n];
        let chars = vec![r, 'A', 'B', 'C'].into_iter().unique().collect_vec();
        for positions in (0..n).combinations(3) {
            let mut line = vec!['.'; n];

            for (&c, &pos) in chars.iter().zip(&positions) {
                line[pos] = c;
            }

            let mut next_board_1 = board.clone();
            next_board_1.extend(line.clone());
            if is_ok(&next_board_1) {
                stack.push(next_board_1);
            }

            line.swap(positions[1], positions[2]);

            let mut next_board_2 = board.clone();
            next_board_2.extend(line);
            if is_ok(&next_board_2) {
                stack.push(next_board_2);
            }
        }
    }

    None
}

use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use rand::{seq::SliceRandom, Rng};

const NUMBER_OF_TESTS: usize = 1000;

fn main() {
    for test_case_index in 0..NUMBER_OF_TESTS {
        let (input, expected_output) = generate();
        let (actual_output, questions_cnt) = solve(&input);

        if questions_cnt > 20 {
            println!("n = {}, questions_cnt = {}", input.len(), questions_cnt);
        }

        if expected_output != actual_output {
            show_board(&input);
        }

        assert_eq!(
            expected_output, actual_output,
            "
Wrong Answer on Test #{}

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
            test_case_index, input, expected_output, actual_output
        );
    }
}

fn show_board(board: &Vec<Vec<bool>>) {
    let n = board.len();

    for i in 0..n {
        for j in 0..n {
            let c = if board[i][j] { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
}

fn generate() -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(2..=1000);

    let mut rem_rows_indexes: Vec<usize> = (0..n).collect();
    rem_rows_indexes.shuffle(&mut rng);

    let mut rem_cols_indexes: Vec<usize> = (0..n).collect();
    rem_cols_indexes.shuffle(&mut rng);

    let mut board = vec![vec![false; n]; n];

    for (&row_idx, &col_idx) in rem_rows_indexes.iter().zip(rem_cols_indexes.iter()).skip(1) {
        board[row_idx][col_idx] = true;
    }

    (board, (rem_rows_indexes[0] + 1, rem_cols_indexes[0] + 1))
}

fn solve(board: &Vec<Vec<bool>>) -> ((usize, usize), usize) {
    let n = board.len();

    let mut process = Command::new("../../target/release/abc269-e")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = process.stdin.take().unwrap();
    let mut stdout = BufReader::new(process.stdout.take().unwrap());

    stdin.write_all(format!("{}\n", n).as_bytes()).unwrap();
    stdin.flush().unwrap();

    let mut questions_cnt = 0_usize;

    loop {
        let mut buf = String::new();
        stdout.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();

        if iter.next().unwrap() == "!" {
            let (row, col) = (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            );

            break ((row, col), questions_cnt);
        }

        let (a, b, c, d) = (
            iter.next().unwrap().parse::<usize>().unwrap() - 1,
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap() - 1,
            iter.next().unwrap().parse::<usize>().unwrap(),
        );

        let rook_cnt: usize = board[a..b]
            .iter()
            .map(|line| line[c..d].iter().filter(|&&x| x).count())
            .sum();

        stdin
            .write_all(format!("{}\n", rook_cnt).as_bytes())
            .unwrap();
        stdin.flush().unwrap();

        questions_cnt += 1;
    }
}

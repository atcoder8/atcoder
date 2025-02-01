use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: String,
    }

    let init_seq = aa
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect_vec();
    let mut sequences = vec![init_seq];
    for i in 0..n {
        let next_seq = sequences[i]
            .chunks(3)
            .map(|chunk| find_majority(chunk))
            .collect_vec();
        sequences.push(next_seq);
    }
    sequences.reverse();

    let ans = rec(&sequences, 0, 0, 1 - sequences[0][0]);
    println!("{}", ans);
}

fn find_majority(triple: &[u8]) -> u8 {
    let mut counts = [0_u8; 2];
    for &a in triple {
        counts[a as usize] += 1;
    }

    (counts[1] > counts[0]) as u8
}

fn rec(sequences: &[Vec<u8>], depth: usize, pos: usize, dest: u8) -> usize {
    let n = sequences.len() - 1;

    if depth == n {
        return (sequences[n][pos] != dest) as usize;
    }

    (3 * pos..3 * pos + 3)
        .map(|next_pos| {
            if sequences[depth + 1][next_pos] == dest {
                return 0;
            }

            rec(sequences, depth + 1, next_pos, dest)
        })
        .sorted_unstable()
        .take(2)
        .sum::<usize>()
}

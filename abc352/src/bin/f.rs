use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, i64); m],
    }

    let orders = solve(n, abc);

    let ans = orders
        .iter()
        .map(|order| match order {
            Some(order) => (order + 1).to_string(),
            None => "-1".to_owned(),
        })
        .join(" ");
    println!("{}", ans);
}

fn solve(n: usize, abc: Vec<(usize, usize, i64)>) -> Vec<Option<usize>> {
    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, -c));
        graph[b].push((a, c));
    }

    let mut pieces = vec![];
    let mut visited = 0_usize;

    for init in 0..n {
        if visited >> init & 1 == 1 {
            continue;
        }

        let mut piece = vec![];
        let mut stack = vec![(init, 0)];

        while let Some((y, potential)) = stack.pop() {
            if visited >> y & 1 == 1 {
                continue;
            }

            visited |= 1 << y;

            piece.push((y, potential));

            stack.extend(graph[y].iter().map(|&(x, c)| (x, potential + c)));
        }

        piece.sort_unstable_by_key(|&(_, potential)| potential);

        let min_potential = piece[0].1;
        piece
            .iter_mut()
            .for_each(|(_, potential)| *potential -= min_potential);

        pieces.push(piece);
    }

    let potential_bits_vec = pieces
        .iter()
        .map(|piece| {
            piece
                .iter()
                .map(|&(_, potential)| 1 << potential)
                .sum::<usize>()
        })
        .collect_vec();

    let find_shift = |target_piece_idx: usize| {
        let mut dp = vec![false; 1 << n];
        dp[0] = true;

        for (piece_idx, (piece, &potential_bits)) in enumerate(izip!(&pieces, &potential_bits_vec))
        {
            for bits in (0..1 << n).rev() {
                if !dp[bits] {
                    continue;
                }

                if piece_idx == target_piece_idx {
                    continue;
                }

                let max_potential = piece.last().unwrap().1 as usize;
                for shift in 0..n - max_potential {
                    let add_bits = potential_bits << shift;
                    if bits & add_bits == 0 {
                        dp[bits | add_bits] = true;
                    }
                }
            }
        }

        let mut shift = None;
        let target_max_potential = pieces[target_piece_idx].last().unwrap().1 as usize;

        for cand_shift in 0..n - target_max_potential {
            let bits = potential_bits_vec[target_piece_idx] << cand_shift;
            let other_bits = (1 << n) - 1 ^ bits;

            if dp[other_bits] {
                if shift.is_some() {
                    return None;
                }

                shift = Some(cand_shift);
            }
        }

        shift
    };

    let mut orders = vec![None; n];
    for (target_piece_idx, piece) in enumerate(&pieces) {
        if let Some(shift) = find_shift(target_piece_idx) {
            for &(person, potential) in piece {
                orders[person] = Some(potential as usize + shift);
            }
        }
    }

    orders
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::{enumerate, Itertools};
    use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

    use crate::solve;

    /// Input data type.
    type Input = (usize, Vec<(usize, usize, i64)>);

    /// Output data type.
    type Output = Vec<Option<usize>>;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;
        const SEED: u64 = 0;

        let mut rng: StdRng = SeedableRng::seed_from_u64(SEED);

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator(&mut rng);
            let expected_output = expected(input.clone());
            let actual_output = actual(input.clone());

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

    /// Generate test cases.
    pub fn generator<R>(rng: &mut R) -> Input
    where
        R: Rng,
    {
        let n = rng.gen_range(2..=7);

        let mut perm = (0..n).collect_vec();
        perm.shuffle(rng);

        let mut edges = (0..n)
            .tuple_combinations()
            .map(|(y, x)| (perm[x], perm[y], (x - y) as i64))
            .collect_vec();
        edges.shuffle(rng);

        let m = rng.gen_range(0..=n * (n - 1) / 2);
        edges.truncate(m);

        (n, edges)
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let (n, abc) = input;

        let is_ok = |perm: &[usize]| {
            for &(a, b, c) in &abc {
                let pos_a = perm.iter().position(|&person| person == a).unwrap();
                let pos_b = perm.iter().position(|&person| person == b).unwrap();

                if pos_a < pos_b || pos_a - pos_b != c as usize {
                    return false;
                }
            }

            true
        };

        let mut cand_each_person = vec![vec![]; n];
        for perm in (0..n).permutations(n) {
            if is_ok(&perm) {
                for (order, &person) in enumerate(&perm) {
                    cand_each_person[person].push(order);
                }
            }
        }

        let orders = cand_each_person
            .iter()
            .map(|cand_orders| {
                if !cand_orders.is_empty() && cand_orders.iter().all_equal() {
                    Some(cand_orders[0])
                } else {
                    None
                }
            })
            .collect_vec();

        orders
    }

    /// Test this program.
    pub fn actual(input: Input) -> Output {
        let (n, abc) = input;

        solve(n, abc)
    }
}

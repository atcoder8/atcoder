use im_rc::HashMap;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let ans = rec(&ab, 0, 0, &mut HashMap::new());
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}

fn rec(
    ab: &[(usize, usize)],
    removed: usize,
    turn: usize,
    memo: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if let Some(&res) = memo.get(&(removed, turn)) {
        return res;
    }

    let rem_cards = (0..ab.len())
        .filter(|&card| removed >> card & 1 == 0)
        .collect_vec();

    let mut res = false;
    for (&card1, &card2) in rem_cards.iter().tuple_combinations() {
        if ab[card1].0 != ab[card2].0 && ab[card1].1 != ab[card2].1 {
            continue;
        }

        res |= !rec(ab, removed | (1 << card1) | (1 << card2), turn + 1, memo);
    }

    memo.insert((removed, turn), res);

    res
}

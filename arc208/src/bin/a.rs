// unfinished
use itertools::Itertools;
use proconio::input;

// 山が1つ -> 負け
// 山が2つ -> 重なりがあれば勝ち、なければ負け

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| if solve() { "Alice" } else { "Bob" })
        .join("\n");
    println!("{output}");
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut bb = [0_usize; 30];
    for &a in &aa {
        for exp in 0..30 {
            bb[exp] += a >> exp & 1;
        }
    }
    bb.iter_mut().for_each(|b| *b -= 1);

    bb.iter().fold(0, |acc, &b| acc ^ b) != 0
}

#[cfg(test)]
mod tests {
    use im_rc::HashMap;
    use itertools::Itertools;

    #[test]
    fn test() {
        const MAX: usize = 6;

        for len in 2..=4 {
            for i in 0..MAX.pow(len) {
                let mut rem = i;
                let stocks = (0..len)
                    .map(|_| {
                        let stock = rem % MAX + 1;
                        rem /= MAX;
                        stock
                    })
                    .collect_vec();
                let mut sorted_stocks = stocks.clone();
                sorted_stocks.sort_unstable();
                if stocks != sorted_stocks {
                    continue;
                }
                let formatted_stocks = stocks.iter().map(|&stock| format!("{stock:03b}")).join(" ");
                println!(
                    "{formatted_stocks}: {}",
                    if expected(&stocks, &mut HashMap::new()) {
                        "Alice"
                    } else {
                        "Bob"
                    }
                );
            }
        }
    }

    fn calc_bitwise_or(sequence: &[usize]) -> usize {
        sequence.iter().fold(0, |acc, elem| acc | elem)
    }

    fn expected(stocks: &[usize], memo: &mut HashMap<Vec<usize>, bool>) -> bool {
        if let Some(win) = memo.get(stocks).copied() {
            return win;
        }

        let bitwise_or = calc_bitwise_or(stocks);

        let validate_stocks = |stocks: &[usize]| calc_bitwise_or(stocks) == bitwise_or;

        for stock_idx in 0..stocks.len() {
            for num_remove_discs in 1..=stocks[stock_idx] {
                let mut next_stocks = stocks.to_vec();
                next_stocks[stock_idx] -= num_remove_discs;
                if validate_stocks(&next_stocks) && !expected(&next_stocks, memo) {
                    memo.insert(next_stocks, true);
                    return true;
                }
            }
        }

        memo.insert(stocks.to_vec(), false);
        false
    }
}

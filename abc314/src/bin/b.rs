use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aaa: [[usize]; n],
        x: usize,
    }

    let mut winners = vec![];
    for (i, aa) in aaa.iter().enumerate() {
        if aa.contains(&x) {
            winners.push(i);
        }
    }
    let min = winners.iter().map(|&winner| aaa[winner].len()).min();
    if let Some(min) = min {
        winners.retain(|&winner| aaa[winner].len() == min);
        println!(
            "{}\n{}",
            winners.len(),
            join(winners.iter().map(|&winner| winner + 1), " ")
        );
    } else {
        println!("0\n");
    }
}

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xx: [i64; n],
    }

    let mut negative = xx
        .iter()
        .filter(|&&x| x.is_negative())
        .cloned()
        .collect_vec();
    negative.sort_unstable();
    let mut positive = xx
        .iter()
        .filter(|&&x| x.is_positive())
        .cloned()
        .collect_vec();
    positive.sort_unstable();

    let mut seq = vec![];

    let negative_num = negative.len();
    if negative_num >= 6 {
        seq.append(&mut negative[..3].to_vec());
        seq.append(&mut negative[(negative_num - 3)..].to_vec());
    } else {
        seq.append(&mut negative.clone());
    }

    let positive_num = positive.len();
    if positive_num >= 6 {
        seq.append(&mut positive[..3].to_vec());
        seq.append(&mut positive[(positive_num - 3)..].to_vec());
    } else {
        seq.append(&mut positive.clone());
    }

    let mut min = std::f64::MAX;
    let mut max = std::f64::MIN;

    let m = seq.len();
    for i in 0..m {
        let a = seq[i] as f64;

        for j in (i + 1)..m {
            let b = seq[j] as f64;

            for k in (j + 1)..m {
                let c = seq[k] as f64;

                let val = (a + b + c) / (a * b * c);
                min = min.min(val);
                max = max.max(val);
            }
        }
    }

    println!("{}\n{}", min, max);
}

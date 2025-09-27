use itertools::Itertools;
use proconio::input;

fn main() {
    let answer = match solve() {
        Some(pp) => format!("Yes\n{}", pp.iter().join(" ")),
        None => "No".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<Vec<i64>> {
    input! {
        n: usize,
        aa: [i64; n],
    }

    if !aa.iter().filter(|&&a| a != -1).all_unique() {
        return None;
    }

    let mut rem = (1..=n as i64).filter(|&i| !aa.contains(&i));
    let mut pp = aa.clone();
    pp.iter_mut().for_each(|p| {
        if *p == -1 {
            *p = rem.next().unwrap()
        }
    });
    Some(pp)
}

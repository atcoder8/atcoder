use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut learned_skills = vec![];
    let mut graph = vec![vec![]; n];
    for (i, &(a, b)) in enumerate(&ab) {
        if a == 0 && b == 0 {
            learned_skills.push(i);
        } else {
            graph[a - 1].push(i);
            graph[b - 1].push(i);
        }
    }

    let mut learned = vec![false; n];
    let mut stack = learned_skills;
    while let Some(skill) = stack.pop() {
        if learned[skill] {
            continue;
        }

        learned[skill] = true;
        stack.extend(graph[skill].iter().cloned());
    }

    let ans = learned.iter().filter(|&&v| v).count();
    println!("{}", ans);
}

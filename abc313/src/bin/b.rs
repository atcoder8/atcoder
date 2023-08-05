use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut indegree = vec![0; n];
    for &(_, b) in &ab {
        indegree[b] += 1;
    }

    if indegree.iter().filter(|&&deg| deg == 0).count() >= 2 {
        println!("-1");
        std::process::exit(0);
    }

    let ans = indegree.iter().position(|&deg| deg == 0).unwrap() + 1;
    println!("{}", ans);
}

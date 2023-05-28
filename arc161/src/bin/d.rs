// unfinished

use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
    }

    if d > (n - 1) / 2 {
        println!("No");
        std::process::exit(0);
    }

    println!("Yes");

    let mut edges: Vec<(usize, usize)> = vec![];

    for i in 0..n {
        for j in 0..d {
            edges.push((i, i + 1 + j));
        }
    }

    for &(node1, node2) in &edges {
        println!("{} {}", node1 + 1, node2 + 1);
    }
}

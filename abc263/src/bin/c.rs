use itertools::{Itertools, join};

fn main() {
    let (n, m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    for comb in (0..m).combinations(n) {
        println!("{} ", join(comb.iter().map(|&x| x + 1), " "));
    }
}

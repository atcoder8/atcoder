fn main() {
    let (r, c): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let mut aaa: Vec<Vec<usize>> = Vec::new();
    for _ in 0..2 {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        aaa.push(
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }

    println!("{}", aaa[r - 1][c - 1]);
}

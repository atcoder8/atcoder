fn main() {
    let (n, q): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let cc: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().collect()
    };

    let mut cnt = 0;
    for _ in 0..q {
        let (query_type, x): (usize, usize) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        };

        if query_type == 1 {
            cnt = (cnt + x) % n;
        } else {
            println!("{}", cc[(n + (x - 1) - cnt) % n]);
        }
    }
}

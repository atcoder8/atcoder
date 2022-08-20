fn main() {
    let _n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let aa: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let mut p = 0;
    let mut squares = vec![0; 4];

    for a in aa {
        squares[0] = 1;
        for i in (0..4).rev() {
            if i + a >= 4 {
                p += squares[i];
            } else {
                squares[i + a] += squares[i];
            }
            squares[i] = 0;
        }
    }

    println!("{}", p);
}

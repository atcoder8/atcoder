fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let pp = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut diff_counts = vec![0; n];

    for (i, &p) in pp.iter().enumerate() {
        let diff = (i + n - p) % n;

        diff_counts[(diff + n - 1) % n] += 1;
        diff_counts[diff] += 1;
        diff_counts[(diff + 1) % n] += 1;
    }

    let ans = diff_counts.iter().max().unwrap();
    println!("{}", ans);
}

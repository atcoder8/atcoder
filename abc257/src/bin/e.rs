fn main() {
    let mut n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let cc: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let min_c = *cc.iter().min().unwrap();
    let length = n / min_c;
    let mut ans = vec![];
    for _ in 0..length {
        for i in (1..10).rev() {
            if n >= cc[i - 1] && ans.len() + 1 + (n - cc[i - 1]) / min_c == length {
                ans.push(i);
                n -= cc[i - 1];
                break;
            }
        }
    }

    ans.iter().for_each(|x| print!("{}", x));
    println!();
}

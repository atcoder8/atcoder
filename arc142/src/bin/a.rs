fn main() {
    let (n, mut k): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut rev_k: usize = k
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();
    if rev_k < k {
        println!("0");
        std::process::exit(0);
    }

    let flag = k != rev_k;

    let mut ans = 0;
    while k <= n {
        ans += 1;
        k *= 10;
    }
    if flag {
        while rev_k <= n {
            ans += 1;
            rev_k *= 10;
        }
    }

    println!("{}", ans);
}

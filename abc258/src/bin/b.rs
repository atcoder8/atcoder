fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let mut aaa: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        aaa.push(
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    let mut ans = 0;
    for h in 0..n {
        for w in 0..n {
            for i in 0..3 {
                for j in 0..3 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    let mut val = 0;
                    for k in 0..n {
                        let curr_h = (n + h + i * k - k) % n;
                        let curr_w = (n + w + j * k - k) % n;
                        val = 10 * val + aaa[curr_h][curr_w];
                    }
                    ans = ans.max(val);
                }
            }
        }
    }

    println!("{}", ans);
}

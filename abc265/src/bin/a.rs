fn main() {
    let (x, y, n) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut ans = 1e18 as usize;
    for i in 0..110 {
        for j in 0..110 {
            if i + 3 * j == n {
                ans = ans.min(x * i + y * j);
            }
        }
    }

    println!("{}", ans);
}

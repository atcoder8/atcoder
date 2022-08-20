fn main() {
    let (n, a, b) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let ans = if n < a {
        0
    } else if a <= b {
        n - a + 1
    } else {
        (n / a - 1) * b + (n % a + 1).min(b)
    };

    println!("{}", ans);
}

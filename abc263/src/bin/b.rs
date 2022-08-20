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
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>()
    };

    let mut cnt = 0;
    let mut curr = n - 1;

    while curr != 0 {
        curr = pp[curr - 1];
        cnt += 1;
    }

    println!("{}", cnt);
}

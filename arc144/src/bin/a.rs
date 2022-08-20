fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };

    let (q, r) = (n / 4, n % 4);

    println!("{}", 2 * n);
    if r > 0 {
        print!("{}", r);
    }
    for _ in 0..q {
        print!("4");
    }
    println!();
}

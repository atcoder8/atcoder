fn main() {
    let k: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };

    println!("{:02}:{:02}", 21 + k / 60, k % 60);
}

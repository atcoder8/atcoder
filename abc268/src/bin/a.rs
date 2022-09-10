fn main() {
    let mut aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    aa.sort_unstable();
    aa.dedup();

    println!("{}", aa.len());
}

use std::{collections::BTreeSet, io::Write};

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    std::io::stdout().flush().unwrap();

    let mut rem: BTreeSet<usize> = (1..=(2 * n + 1)).collect();
    while let Some(a) = rem.pop_first() {
        println!("{}", a);
        std::io::stdout().flush().unwrap();

        let b = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<usize>().unwrap()
        };
        rem.remove(&b);
    }
}

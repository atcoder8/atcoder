fn main() {
    let cc: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().collect()
    };

    if cc[0] != cc[1] && cc[0] != cc[2] {
        println!("{}", cc[0]);
    } else if cc[1] != cc[0] && cc[1] != cc[2] {
        println!("{}", cc[1]);
    } else if cc[2] != cc[0] && cc[2] != cc[1] {
        println!("{}", cc[2]);
    } else {
        println!("-1");
    }
}

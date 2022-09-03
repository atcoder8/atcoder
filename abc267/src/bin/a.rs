fn main() {
    let week = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    let s = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<String>().unwrap()
    };

    for (i, x) in week.iter().enumerate() {
        if x == &s {
            println!("{}", 5 - i);
            break;
        }
    }
}

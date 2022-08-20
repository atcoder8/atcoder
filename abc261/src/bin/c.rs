use std::collections::HashMap;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    let mut map = HashMap::new();

    for _ in 0..n {
        let s = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_string()
        };

        if map.contains_key(&s) {
            let value = map.get_mut(&s).unwrap();
            println!("{}({})", s, value);
            *value += 1;
        } else {
            println!("{}", s);
            map.insert(s, 1);
        }
    }
}

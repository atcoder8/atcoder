use proconio::input;

fn main() {
    input! {
        d: String,
    }

    let ans = d.chars().map(opposite).collect::<String>();
    println!("{}", ans);
}

fn opposite(c: char) -> char {
    match c {
        'N' => 'S',
        'E' => 'W',
        'W' => 'E',
        'S' => 'N',
        _ => panic!(),
    }
}

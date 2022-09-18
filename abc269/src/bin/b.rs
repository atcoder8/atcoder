use proconio::{input, marker::Chars};

fn main() {
    input! {
        ss: [Chars; 10],
    }

    let start = start(&ss);
    let end = end(&ss);

    println!("{} {}\n{} {}", start.0, end.0, start.1, end.1);
}

fn start(ss: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..10 {
        for j in 0..10 {
            if ss[i][j] == '#' {
                return (i + 1, j + 1);
            }
        }
    }

    panic!()
}

fn end(ss: &Vec<Vec<char>>) -> (usize, usize) {
    for i in (0..10).rev() {
        for j in (0..10).rev() {
            if ss[i][j] == '#' {
                return (i + 1, j + 1);
            }
        }
    }

    panic!()
}

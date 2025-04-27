// unfinished
#![allow(dead_code)]

use proconio::input;

// 括弧の外側に加算が含まれている式に対して積をとる場合のみ括弧を追加する必要がある

fn main() {
    input! {
        n: usize,
    }

    println!("{}", recursion(n).0);
}

fn recursion(n: usize) -> (String, bool) {
    if [1, 11, 111, 1111].contains(&n) {
        return (n.to_string(), false);
    }

    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Formula {
    formula: String,
    complex: bool,
}

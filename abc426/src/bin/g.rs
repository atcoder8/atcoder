// use proconio::input;
use ndarray::{prelude::*, stack};

fn main() {
    // input! {

    // }

    let v1 = Array1::from_vec(vec![1, 2]);
    let v2 = Array1::from_vec(vec![3, 4]);
    let a = stack(Axis(1), &[v1.view(), v2.view()]).unwrap();
    println!("{a}");
}

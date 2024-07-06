use proconio::input;

fn main() {
    input! {
        rect1: ([usize; 3], [usize; 3]),
        rect2: ([usize; 3], [usize; 3]),
    }

    let ans = (0..3).all(|axis| rect1.0[axis] < rect2.1[axis] && rect2.0[axis] < rect1.1[axis]);
    println!("{}", if ans { "Yes" } else { "No" });
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut stack = vec![];
    for &a in &aa {
        stack.push(a);

        while stack.len() >= 2 && stack[stack.len() - 1] == stack[stack.len() - 2] {
            let size = stack.pop().unwrap();
            stack.pop();
            stack.push(size + 1);
        }
    }

    println!("{}", stack.len());
}

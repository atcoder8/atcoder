use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut pp: [Usize1; n],
    }

    let mut operations = vec![];

    for i in 0..n {
        if pp[i] % 2 != i % 2 {
            let mut pos = i;
            while pos >= 2 && pp[pos - 2] % 2 == pos % 2 {
                operations.push(('B', pos - 2));
                pp.swap(pos - 2, pos);
                pos -= 2;
            }
        }
    }

    for i in (0..n).step_by(2) {
        if pp[i] % 2 == i % 2 {
            break;
        }

        operations.push(('A', i));
        pp.swap(i, i + 1);
    }

    for i in 0..n {
        let mut pos = pp.iter().position(|&x| x == i).unwrap();
        while pos != i {
            operations.push(('B', pos - 2));
            pp.swap(pos - 2, pos);
            pos -= 2;
        }
    }

    println!("{}", operations.len());
    for &(op, pos) in operations.iter() {
        println!("{} {}", op, pos + 1);
    }
}

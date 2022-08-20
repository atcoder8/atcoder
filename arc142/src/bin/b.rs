fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };

    let mut mat = vec![vec![0_usize; n]; n];
    let mut cnt = 1;
    for i in (0..n).step_by(2) {
        for j in 0..n {
            mat[i][j] = cnt;
            cnt += 1;
        }
    }
    for i in (1..n).step_by(2) {
        for j in 0..n {
            mat[i][j] = cnt;
            cnt += 1;
        }
    }

    for vec in mat.iter() {
        vec.iter().for_each(|x| print!("{} ", x));
        println!();
    }
}

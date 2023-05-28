use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let odd_num = (n + 1) / 2;
    let even_num = n - odd_num;

    let mut bb = vec![0; n];
    for i in 0..odd_num {
        bb[2 * i] = aa[i];
    }
    for i in 0..even_num {
        bb[2 * i + 1] = aa[odd_num + i];
    }

    let ans = (1..n)
        .step_by(2)
        .all(|i| bb[i - 1] < bb[i] && bb[i] > bb[i + 1]);
    println!("{}", if ans { "Yes" } else { "No" });
}

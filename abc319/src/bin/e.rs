use proconio::input;

const LCM: usize = 840;

fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        pt: [(usize, usize); n - 1],
        query_num: usize,
        qq: [usize; query_num],
    }

    let mut req_times = vec![0; LCM];
    for start in 0..LCM {
        let mut time = start;
        time += x;
        for &(p, t) in &pt {
            time = (time + p - 1) / p * p + t;
        }
        time += y;

        req_times[start] = time - start;
    }

    for &q in &qq {
        println!("{}", q + req_times[q % LCM]);
    }
}

use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        pt: [(usize, usize); n - 1],
        query_num: usize,
        qq: [usize; query_num],
    }

    let mut req_times = vec![0; 840];
    for start in 0..840 {
        let mut time = start;
        time += x;
        for &(p, t) in &pt {
            if time % p != 0 {
                time += p - (time % p);
            }
            time += t;
        }
        time += y;

        req_times[start] = time - start;
    }

    for &q in &qq {
        println!("{}", q + req_times[q % 840]);
    }
}

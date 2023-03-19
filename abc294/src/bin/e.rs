use proconio::input;

fn main() {
    input! {
        (_l, n1, n2): (usize, usize, usize),
        vl1: [(usize, usize); n1],
        vl2: [(usize, usize); n2],
    }

    let mut ans = 0;
    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut pos1 = 0;
    let mut pos2 = 0;
    let mut val = 0;

    while idx1 < n1 || idx2 < n2 {
        if pos1 <= pos2 {
            let (v1, l1) = vl1[idx1];

            if v1 == val {
                ans += l1.min(pos2 - pos1);
            }

            pos1 += l1;

            if pos1 > pos2 {
                val = v1;
            }

            idx1 += 1;
        } else {
            let (v2, l2) = vl2[idx2];

            if v2 == val {
                ans += l2.min(pos1 - pos2);
            }

            pos2 += l2;

            if pos2 > pos1 {
                val = v2;
            }

            idx2 += 1;
        }
    }

    println!("{}", ans);
}

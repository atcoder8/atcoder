use itertools::join;
use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
        q: usize,
        ab: [(usize, usize); q],
    }

    let mut v_slide_cnt = 0;
    let mut h_slide_cnt = 0;

    for (i, &(a, b)) in ab.iter().enumerate() {
        if i % 2 == 0 {
            v_slide_cnt += h - a;
            h_slide_cnt += w - b;
        } else {
            v_slide_cnt += a;
            h_slide_cnt += b;
        }
    }

    v_slide_cnt %= h;
    h_slide_cnt %= w;

    let mut slide_aaa = vec![vec!['\0'; w]; h];

    for i in 0..v_slide_cnt {
        for j in 0..h_slide_cnt {
            slide_aaa[i][j] = aaa[h - v_slide_cnt + i][w - h_slide_cnt + j];
        }

        for j in h_slide_cnt..w {
            slide_aaa[i][j] = aaa[h - v_slide_cnt + i][j - h_slide_cnt];
        }
    }

    for i in v_slide_cnt..h {
        for j in 0..h_slide_cnt {
            slide_aaa[i][j] = aaa[i - v_slide_cnt][w - h_slide_cnt + j];
        }

        for j in h_slide_cnt..w {
            slide_aaa[i][j] = aaa[i - v_slide_cnt][j - h_slide_cnt];
        }
    }

    if q % 2 == 1 {
        slide_aaa.iter_mut().for_each(|x| x.reverse());
        slide_aaa.reverse();
    }

    for x in slide_aaa {
        println!("{}", join(x, ""));
    }
}

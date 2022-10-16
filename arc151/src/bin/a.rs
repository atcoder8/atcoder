use proconio::{input, marker::Chars};

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<String> {
    input! {
        n: usize,
        mut s: Chars,
        mut t: Chars,
    }

    let mut s_one_num = s.iter().filter(|&&c| c == '1').count();
    let mut t_one_num = t.iter().filter(|&&c| c == '1').count();

    if s_one_num > t_one_num {
        std::mem::swap(&mut s, &mut t);
        std::mem::swap(&mut s_one_num, &mut t_one_num);
    }

    let diff_num = t_one_num - s_one_num;

    if diff_num % 2 == 1 {
        return None;
    }

    let mut cnt = s
        .iter()
        .zip(t.iter())
        .map(|(&c1, &c2)| {
            if c1 == '0' && c2 == '1' {
                1_i64
            } else if c1 == '1' && c2 == '0' {
                -1_i64
            } else {
                0_i64
            }
        })
        .sum::<i64>() as usize;

    let mut ans = vec!['0'; n];

    for i in (0..n).rev() {
        if cnt == 0 {
            break;
        }

        if s[i] == '0' && t[i] == '1' {
            ans[i] = '1';
            cnt -= 2;
        }
    }

    Some(ans.into_iter().collect())
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        mut s: Chars,
    }

    let open_num = s.iter().filter(|&&c| c == '(').count();
    let close_num = 2 * n - open_num;

    let mut replaced_num = 0;

    if open_num < close_num {
        let mut rem = (close_num - open_num) / 2;
        replaced_num += rem;

        for i in 0..2 * n {
            if s[i] == ')' {
                s[i] = '(';
                rem -= 1;

                if rem == 0 {
                    break;
                }
            }
        }
    }

    let mut open = 0;
    let mut swap_num = 0;
    let mut iter = (0..2 * n).rev();
    for i in 0..2 * n {
        if s[i] == '(' {
            open += 1;
            continue;
        }

        if open == 0 {
            let open_pos = iter.find(|&i| s[i] == '(');
            if open_pos.is_some_and(|open_pos| open_pos > i) {
                s.swap(i, open_pos.unwrap());
                open += 1;
                swap_num += 1;
            } else {
                s[i] = '(';
                open += 1;
                replaced_num += 1;
            }
        } else {
            open -= 1;
        }
    }
    replaced_num += open / 2;

    let ans = (swap_num * a + replaced_num * b).min((2 * swap_num + replaced_num) * b);
    println!("{}", ans);
}

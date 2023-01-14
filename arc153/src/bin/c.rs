// unfinished

use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [i64; n],
    }

    if n == 1 {
        return true;
    }

    let positive_num = aa.iter().filter(|&&a| a == 1).count();
    let negative_num = n - positive_num;

    // 0を含む
    let left_pos_num = 0;
    let left_neg_num = 0;

    let right_pos_num = positive_num - (aa[0] == 1) as usize;
    let right_neg_num = negative_num - (aa[0] == -1) as usize;

    for zero_idx in 0..n {
        let left_pos_edge_win = aa[0] == -1;
        let left_pos_num_win = left_pos_num < left_neg_num;

        let left_neg_edge_win = aa[0] == 1;
        let left_neg_num_win = left_pos_num > left_neg_num;

        let left_pos_win = aa[0] == -1 || left_pos_num < left_neg_num;
        let left_neg_win = aa[0] == 1 || left_pos_num > left_neg_num;

        let right_pos_win = aa[n - 1] == 1 || right_pos_num > right_neg_num;
        let right_neg_win = aa[n - 1] == -1 || right_pos_num < right_neg_num;

        if left_pos_win && right_neg_win {
            
        }
    }

    if aa[n - 1] == 1
        && (positive_num < negative_num || (aa[0] == 1 && positive_num == negative_num))
    {
        return true;
    }

    if aa[n - 1] == -1
        && (positive_num > negative_num || (aa[0] == -1 && positive_num == negative_num))
    {
        return true;
    }

    if aa[0] == 1 && positive_num > negative_num || aa[0] == -1 && positive_num < negative_num {
        return true;
    }

    let mut left_negative_num = 0;
    let mut left_positive_num = 0;

    let mut right_negative_num = n - positive_num;
    let mut right_positive_num = positive_num;

    for i in 0..(n - 1) {
        left_negative_num += (aa[i] == -1) as usize;
        left_positive_num += (aa[i] == 1) as usize;

        right_negative_num -= (aa[i] == -1) as usize;
        right_positive_num -= (aa[i] == 1) as usize;

        let left_pos_win = aa[0] == -1 || left_positive_num < left_negative_num;
        let left_neg_win = aa[0] == 1 || left_positive_num > left_negative_num;

        let right_pos_win = aa[n - 1] == 1
            || right_positive_num > right_negative_num
            || (right_positive_num == right_negative_num && aa[i + 1] == -1);
        let right_neg_win = aa[n - 1] == -1
            || (right_positive_num < right_negative_num)
            || (right_positive_num == right_negative_num && aa[i + 1] == 1);

        if left_pos_win && right_neg_win || left_neg_win && right_pos_win {
            return true;
        }
    }

    false
}

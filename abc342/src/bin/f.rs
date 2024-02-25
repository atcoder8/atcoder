use proconio::input;

fn main() {
    input! {
        (n, l, d): (usize, usize, usize),
    }

    let mut imos = vec![0.0; l + d + 1];
    imos[0] = 1.0;
    imos[1] = -1.0;
    let mut sum = 0.0;
    let mut prob_each_y = vec![0.0; (l + d).max(n + 1)];
    for y in 0..l + d {
        sum += imos[y];

        if y < l {
            let add_prob = sum as f64 / d as f64;
            imos[y + 1] += add_prob;
            imos[y + d + 1] -= add_prob;
        } else {
            prob_each_y[y] = sum;
        }
    }

    let mut prefix_sum_prob_each_y = vec![0.0; l + d + 1];
    for y in l..l + d {
        prefix_sum_prob_each_y[y + 1] += prob_each_y[y] + prefix_sum_prob_each_y[y];
    }

    let mut lower_prob = prob_each_y.iter().sum::<f64>();
    let mut max_win_prob_each_x = vec![0.0; n + d + 1];
    let mut sum_draw_prob = 0.0;
    for x in (0..=n).rev() {
        lower_prob -= prob_each_y[x];
        max_win_prob_each_x[x] = lower_prob.max(sum_draw_prob / d as f64);
        sum_draw_prob += max_win_prob_each_x[x] - max_win_prob_each_x[x + d];
    }

    println!("{}", max_win_prob_each_x[0]);
}

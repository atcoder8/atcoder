use proconio::{input, marker::Chars};

const N: usize = 9;

fn main() {
    input! {
        s: [Chars; N],
    }

    let mut ans = 0;

    for top_x in 0..N {
        for top_y in 0..N {
            if s[top_x][top_y] == '.' {
                continue;
            }

            for down_num in 1..N {
                for left_num in 0..9 {
                    if left_num > top_y || top_y + down_num >= N || top_x + down_num + left_num >= N
                    {
                        continue;
                    }

                    // 真下または左下の点
                    let lower_left_x = top_x + down_num;
                    let lower_left_y = top_y - left_num;

                    if s[lower_left_x][lower_left_y] == '.' {
                        continue;
                    }

                    // 右または右下の点
                    let lower_right_x = top_x + left_num;
                    let lower_right_y = top_y + down_num;

                    if s[lower_right_x][lower_right_y] == '.' {
                        continue;
                    }

                    // 下の点
                    let bottom_x = top_x + down_num + left_num;
                    let bottom_y = lower_left_y + down_num;

                    if s[bottom_x][bottom_y] == '.' {
                        continue;
                    }

                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}

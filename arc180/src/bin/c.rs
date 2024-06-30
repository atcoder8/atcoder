use ac_library::ModInt1000000007;
use itertools::Itertools;
use proconio::input;

type Mint = ModInt1000000007;

fn main() {
    // 標準入力から整数列を読み取る
    input! {
        n: usize,
        aa: [isize; n],
    }

    // 部分和の下限
    let low_lim = aa.iter().filter(|&&a| a < 0).sum::<isize>();
    // 部分和の上限
    let upp_lim = aa.iter().filter(|&&a| a > 0).sum::<isize>();
    // 部分和の下限と上限の差
    let gap = (upp_lim - low_lim) as usize;

    // 差分を対応する添字へ変換するクロージャ
    // 差分diffから部分和の下限を引くことで0以上gap以下の整数にする
    let diff_to_idx = |diff: isize| {
        assert!(low_lim <= diff && diff <= upp_lim);

        (diff - low_lim) as usize
    };

    // dp[i][j]: 操作後のAのうち、変化する最後の要素のインデックスがi(0-based)であり、かつその差分に対応する添字がjであるものの個数
    // A_0が操作後に変化するような操作方法はないので、dp[0]の要素は全て0
    let mut dp: Vec<Vec<_>> = vec![vec![Mint::new(0); gap + 1]];

    // right=1,2,...,N-1に対し、操作後にA_{right}が変化する操作方法の個数を数え上げる
    for right in 1..n {
        // 最後にA_{right}が変化する操作方法の個数をその差分ごとに格納する
        let mut to = vec![Mint::new(0); gap + 1];

        // 初めて変化する要素がA_{right}である操作方法の個数を計算する
        // v∈{A_0,A_1,...,A_{right-1}}∖{0}に対し、部分列として(v,A_{right})を選んで操作した場合、A_{right}のみA_{right}+vに変化する
        for &v in aa[..right].iter().unique() {
            if v != 0 {
                to[diff_to_idx(v)] += 1;
            }
        }

        // A_{left}が変化した直後にA_{right}が変化する操作方法の個数を計算する
        for left in 0..right {
            // 最後にA_{left}が変化する操作方法の個数
            let from = &dp[left];

            // 末尾がA_{left}である総和が0である部分列からの遷移を考える
            // v∈{A_0,A_1,...,A_{right-1}}∖{0}に対し、A_{left}とA_{right}の間にv(≠0)を挟んだ部分列を選んだ場合、A_{left}が変化した直後にA_{right}がA_{right}+vに変化する
            // 部分和にvを入れる前に0である要素を入れることもできるが、操作後の数列の変化には寄与しない
            if low_lim <= -aa[left] && -aa[left] <= upp_lim {
                for &v in aa[left + 1..right].iter().unique() {
                    if v != 0 {
                        to[diff_to_idx(v)] += from[diff_to_idx(-aa[left])];
                    }
                }
            }

            // 末尾がA_{left}である総和が0でない部分列からの遷移を考える
            // 部分和の要素としてA_{left}の直後にA_{right}を選ぶ場合のみA_{left}が変化した直後にA_{right}が変化する
            for diff in low_lim..=upp_lim {
                if diff + aa[left] != 0 {
                    let next_diff = diff + aa[left];
                    if low_lim <= next_diff && next_diff <= upp_lim {
                        to[diff_to_idx(next_diff)] += from[diff_to_idx(diff)];
                    }
                }
            }
        }

        // 最後にA_{right}が変化する操作方法の個数をdpに追加する
        dp.push(to);
    }

    // 最後に変化する要素の位置とその差分は自由なのでDPの全ての要素の総和をとり、操作後に数列が変化しない場合の1通りを加算する
    // 数列が変化しない操作は長さ1の部分列を選ぶことで必ず実現できる
    let ans = dp.iter().flatten().sum::<Mint>() + 1;
    println!("{}", ans);
}

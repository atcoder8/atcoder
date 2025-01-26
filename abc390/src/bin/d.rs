use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    // 各部分集合について総和を計算
    let mut subset_sum = vec![0_usize; 1 << n];
    for bits in 1_usize..1 << n {
        let lsb = bits & bits.wrapping_neg();
        subset_sum[bits] = subset_sum[bits ^ lsb] + aa[lsb.trailing_zeros() as usize];
    }

    // DFSを用いてあり得る値のリストを生成
    let mut xor_list = vec![];
    let init_node = Node::new((1 << n) - 1, 0);
    let mut stack = vec![init_node];
    while let Some(node) = stack.pop() {
        let Node { rem_bits, xor } = node;

        if rem_bits == 0 {
            xor_list.push(xor);
            continue;
        }

        let lsb = rem_bits & rem_bits.wrapping_neg();
        let mut sub_bits = rem_bits;
        while sub_bits != 0 {
            // 残りの要素のうち最も添字の小さいものを必ず選ぶ
            if sub_bits & lsb != 0 {
                let next_node = Node::new(rem_bits ^ sub_bits, xor ^ subset_sum[sub_bits]);
                stack.push(next_node);
            }

            sub_bits = (sub_bits - 1) & rem_bits;
        }
    }

    // 重複を除去
    xor_list.sort_unstable();
    xor_list.dedup();

    println!("{}", xor_list.len());
}

#[derive(Debug, Clone, Copy)]
struct Node {
    rem_bits: usize,
    xor: usize,
}

impl Node {
    fn new(rem_bits: usize, xor: usize) -> Self {
        Self { rem_bits, xor }
    }
}

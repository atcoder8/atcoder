use proconio::input;

fn main() {
    input! {
        n: usize,
        ll: [u8; n],
    }

    let left = ll.iter().take_while(|&&l| l == 0).count() + 1;
    let right = ll.iter().rev().take_while(|&&l| l == 0).count() + 1;
    println!("{}", (n + 1).saturating_sub(left + right));
}

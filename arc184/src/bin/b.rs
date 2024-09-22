// unfinished

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let require = ((1_u128 << n) - 1) << 1;

    let is_ok = |bits: usize| {
        let mut s = 0_u128;
        for x in 1_usize..=n {
            if bits >> x & 1 == 1 {
                for mul in 1..=3 {
                    s |= 1 << x * mul;
                }
            }
        }

        s & require == require
    };

    let ans = (1_usize..1 << n)
        .filter_map(|bits| {
            if is_ok(bits << 1) {
                Some(bits.count_ones())
            } else {
                None
            }
        })
        .min()
        .unwrap();
    println!("{}", ans);
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use rand::{rngs::ThreadRng, Rng};

    /// Input data type.
    type Input = usize;

    /// Output data type.
    type Output = usize;

    /// Performs the specified number of tests.
    #[test]
    fn test() {
        /// This value specifies the number of tests.
        const NUMBER_OF_TESTS: usize = 1000;

        let mut rng = rand::thread_rng();

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator(&mut rng);
            let expected_output = expected(input.clone());
            let actual_output = actual(input.clone());

            // If an unexpected answer is returned, panic is triggered.
            assert_eq!(
                expected_output, actual_output,
                "
Unexpected answer was returned in test case #{}.

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
                test_case_index, input, expected_output, actual_output
            );
        }
    }

    /// Generates a test case.
    pub fn generator(rng: &mut ThreadRng) -> Input {
        rng.gen_range(1..=15)
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let n = input;
        let require = ((1_usize << n) - 1) << 1;

        let is_ok = |bits: usize| {
            let mut s = 0_usize;
            for x in 1_usize..=n {
                if bits >> x & 1 == 1 {
                    for mul in 1..=3 {
                        s |= 1 << x * mul;
                    }
                }
            }

            s & require == require
        };

        (1_usize..1 << n)
            .filter_map(|bits| {
                if is_ok(bits << 1) {
                    Some(bits.count_ones())
                } else {
                    None
                }
            })
            .min()
            .unwrap() as usize
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let n = input;

        expected(n)
    }
}

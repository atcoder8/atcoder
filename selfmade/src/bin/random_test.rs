fn main() {}

/// Module for testing
#[cfg(test)]
mod random_test {
    /// Tuple of input data
    #[derive(Debug, Clone)]
    pub struct Input();

    /// Tuple of Output data
    #[derive(Debug, PartialEq, Eq)]
    pub struct Output();

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let jury_output = jury(input.clone());
            let solve_output = solve(input.clone());

            assert_eq!(
                jury_output, solve_output,
                "\
Wrong Answer on Test #{}

[Input]
{:?}

[Output(Jury)]
{:?}

[Output(Solve)]
{:?}
",
                test_case_index, input, jury_output, solve_output
            );
        }
    }

    /// Generate test cases.
    pub fn generator() -> Input {
        // let rng = rand::thread_rng();

        Input()
    }

    /// Returns the correct answer.
    pub fn jury(input: Input) -> Output {
        // Unpacking input.
        let Input() = input;

        Output()
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        // Unpacking input.
        let Input() = input;

        Output()
    }
}

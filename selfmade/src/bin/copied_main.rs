use factorial::Factorial;
use num::rational::BigRational;
use num_bigint::BigInt;
use num_traits::One;

fn main() {
    const MAX_C_INDEX: usize = 10;

    let mut fac = Factorial::<BigInt>::new();
    let c = calc_c(MAX_C_INDEX + 1, &mut fac);
    c.iter().for_each(|x| println!("{}", x));
}

pub mod factorial {
    use std::ops::{Div, Mul};

    pub struct Factorial<T> {
        fac: Vec<T>,
    }

    impl<T> Default for Factorial<T>
    where
        T: Clone + From<usize> + Mul<Output = T> + Div<Output = T>,
    {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Factorial<T>
    where
        T: Clone + From<usize> + Mul<Output = T> + Div<Output = T>,
    {
        /// Constructs a new, empty `Factorial<T>`.
        pub fn new() -> Self {
            Self {
                fac: vec![T::from(1)],
            }
        }

        /// Returns the factorial of n.
        pub fn factorial(&mut self, n: usize) -> T {
            if self.fac.len() < n + 1 {
                for i in (self.fac.len() - 1)..n {
                    self.fac.push(self.fac[i].clone() * (i + 1).into());
                }
            }
            self.fac[n].clone()
        }

        /// Returns the number of choices when selecting k from n and arranging them in a row.
        pub fn permutation(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / self.factorial(n - k)
            }
        }

        /// Returns the number of choices to select k from n.
        pub fn combination(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / (self.factorial(k) * self.factorial(n - k))
            }
        }
    }
}

fn calc_c(size: usize, fac: &mut Factorial<BigInt>) -> Vec<BigRational> {
    let mut c: Vec<BigRational> = vec![];
    for i in 0..size {
        let v1 = BigRational::new(One::one(), fac.factorial(i + 2).clone());
        let v2 = (0..i)
            .map(|j| {
                BigRational::new(
                    if j % 2 == 0 {
                        BigInt::one()
                    } else {
                        -BigInt::one()
                    },
                    fac.factorial(i + 1 - j),
                ) * c[j].clone()
            })
            .sum::<BigRational>();
        c.push(if i % 2 == 0 { v1 - v2 } else { v2 - v1 });
    }
    c
}

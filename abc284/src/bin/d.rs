use num_integer::Roots;
use proconio::input;

use crate::atcoder8_library::eratosthenes_sieve::EratosthenesSieve;

fn main() {
    let pq = solve();

    for &(p, q) in &pq {
        println!("{} {}", p, q);
    }
}

fn solve() -> Vec<(usize, usize)> {
    input! {
        t: usize,
    }

    let sieve = EratosthenesSieve::new(2_500_000);

    let mut primes = vec![];
    for i in 2..=2_500_000 {
        if sieve.is_prime(i) {
            primes.push(i);
        }
    }

    let mut ret = vec![];

    for _ in 0..t {
        input! {
            n: usize,
        }

        for &p in &primes {
            if n % p == 0 {
                if n / p % p == 0 {
                    ret.push((p, n / p / p));
                } else {
                    ret.push(((n / p).sqrt(), p));
                }

                break;
            }
        }
    }

    ret
}

pub mod atcoder8_library {
    pub mod eratosthenes_sieve {
        //! Implements the Sieve of Eratosthenes.
        //!
        //! Finds the smallest prime factor of each number placed on the sieve,
        //! so it can perform Prime Factorization as well as Primality Test.

        #[derive(Debug, Clone)]
        pub struct EratosthenesSieve {
            sieve: Vec<usize>,
        }

        impl EratosthenesSieve {
            /// Constructs a Sieve of Eratosthenes.
            ///
            /// # Arguments
            ///
            /// * `upper_limit` - The largest number placed on the sieve.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
            ///
            /// let sieve = EratosthenesSieve::new(27);
            /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
            /// ```
            pub fn new(upper_limit: usize) -> Self {
                let mut sieve: Vec<usize> = (0..=upper_limit).collect();

                for p in (2..).take_while(|&i| i * i <= upper_limit) {
                    if sieve[p] != p {
                        continue;
                    }

                    for i in ((p * p)..=upper_limit).step_by(p) {
                        if sieve[i] == i {
                            sieve[i] = p;
                        }
                    }
                }

                Self { sieve }
            }

            /// Returns the least divisor of `n`.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
            ///
            /// let sieve = EratosthenesSieve::new(27);
            /// assert_eq!(sieve.min_divisor(1), 1);
            /// assert_eq!(sieve.min_divisor(2), 2);
            /// assert_eq!(sieve.min_divisor(6), 2);
            /// assert_eq!(sieve.min_divisor(11), 11);
            /// assert_eq!(sieve.min_divisor(27), 3);
            /// ```
            pub fn min_divisor(&self, n: usize) -> usize {
                assert_ne!(n, 0, "`n` must be at least 1.");

                self.sieve[n]
            }

            /// Determines if `n` is prime.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
            ///
            /// let sieve = EratosthenesSieve::new(27);
            /// assert!(!sieve.is_prime(1));
            /// assert!(sieve.is_prime(2));
            /// assert!(!sieve.is_prime(6));
            /// assert!(sieve.is_prime(11));
            /// assert!(!sieve.is_prime(27));
            /// ```
            pub fn is_prime(&self, n: usize) -> bool {
                n >= 2 && self.sieve[n] == n
            }

            /// Performs prime factorization of `n`.
            ///
            /// The result of the prime factorization is returned as a
            /// list of prime factor and exponent pairs.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
            ///
            /// let sieve = EratosthenesSieve::new(27);
            /// assert_eq!(sieve.prime_factorization(1), vec![]);
            /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
            /// assert_eq!(sieve.prime_factorization(19), vec![(19, 1)]);
            /// assert_eq!(sieve.prime_factorization(27), vec![(3, 3)]);
            /// ```
            pub fn prime_factorization(&self, n: usize) -> Vec<(usize, usize)> {
                assert_ne!(n, 0, "`n` must be at least 1.");

                let mut factors: Vec<(usize, usize)> = vec![];
                let mut t = n;

                while t != 1 {
                    let p = self.sieve[t];

                    if factors.is_empty() || factors.last().unwrap().0 != p {
                        factors.push((p, 1));
                    } else {
                        factors.last_mut().unwrap().1 += 1;
                    }

                    t /= p;
                }

                factors
            }

            /// Creates a list of divisors of `n`.
            ///
            /// The divisors are listed in ascending order.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
            ///
            /// let sieve = EratosthenesSieve::new(27);
            /// assert_eq!(sieve.create_divisors_list(1), vec![1]);
            /// assert_eq!(sieve.create_divisors_list(12), vec![1, 2, 3, 4, 6, 12]);
            /// assert_eq!(sieve.create_divisors_list(19), vec![1, 19]);
            /// assert_eq!(sieve.create_divisors_list(27), vec![1, 3, 9, 27]);
            /// ```
            pub fn create_divisors_list(&self, n: usize) -> Vec<usize> {
                assert_ne!(n, 0, "`n` must be at least 1.");

                let prime_factors = self.prime_factorization(n);

                let mut divisors = vec![1];
                let divisors_num: usize = prime_factors.iter().map(|&(_, e)| e + 1).product();
                divisors.reserve(divisors_num - 1);

                for (p, e) in prime_factors {
                    let mut add_divisors = vec![];
                    add_divisors.reserve(divisors.len() * e);
                    let mut mul = 1;

                    for _ in 1..=e {
                        mul *= p;

                        for &d in divisors.iter() {
                            add_divisors.push(d * mul);
                        }
                    }

                    divisors.append(&mut add_divisors);
                }

                divisors.sort_unstable();

                divisors
            }
        }
    }
}

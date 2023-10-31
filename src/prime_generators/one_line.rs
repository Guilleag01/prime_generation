use crate::prime_generators::PrimeGenerator;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub struct OneLine;

impl PrimeGenerator for OneLine {
    fn get_name(&self) -> &str {
        "One line"
    }

    // Autoformatted, it is one line
    fn get_primes(&self, limit: usize) -> Vec<usize> {
        (2..limit)
            .filter(|v| {
                (2..=(*v as f32).sqrt() as usize)
                    .fold_while(true, |acc, x| {
                        if v % x != 0 {
                            Continue(acc)
                        } else {
                            Done(false)
                        }
                    })
                    .into_inner()
            })
            .collect()
    }

    fn new() -> Self {
        Self {}
    }
}

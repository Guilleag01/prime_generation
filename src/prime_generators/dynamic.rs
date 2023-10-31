use crate::prime_generators::PrimeGenerator;

pub struct Dynamic;

impl PrimeGenerator for Dynamic {
    fn get_name(&self) -> &str {
        "Dynamic"
    }

    fn get_primes(&self, limit: usize) -> Vec<usize> {
        let mut found = vec![2];
        'outer: for i in (3..limit).step_by(2) {
            for f in &found {
                if i % f == 0 {
                    continue 'outer;
                }
            }
            found.push(i);
        }
        found
    }

    fn new() -> Self {
        Self {}
    }
}

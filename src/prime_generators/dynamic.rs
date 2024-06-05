use crate::prime_generators::PrimeGenerator;

pub struct Dynamic;

impl PrimeGenerator for Dynamic {
    fn get_name(&self) -> &str {
        "Dynamic"
    }

    fn get_primes(&self, limit: usize) -> Vec<usize> {
        let mut found = vec![2];
        'outer: for i in 3..limit {
            for f in &found {
                if *f as f32 > f32::sqrt(i as f32) {
                    continue 'outer;
                }
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

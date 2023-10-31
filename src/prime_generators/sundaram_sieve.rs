use crate::prime_generators::PrimeGenerator;

pub struct SundaramSieve;

impl PrimeGenerator for SundaramSieve {
    fn get_name(&self) -> &str {
        "Sundaram sieve"
    }

    fn get_primes(&self, limit: usize) -> Vec<usize> {
        let mut nums: Vec<usize> = (1..(limit / 2)).collect();

        'outer: for j in 1..limit {
            for i in 1..=j {
                let k = i + j + 2 * i * j;
                if k > limit / 2 - 1 {
                    continue 'outer;
                }
                nums[k - 1] = 0;
            }
        }

        nums.iter()
            .filter_map(|x| if x == &0 { None } else { Some(2 * x + 1) })
            .collect()
    }

    fn new() -> Self {
        Self {}
    }
}

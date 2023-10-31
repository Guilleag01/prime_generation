use crate::prime_generators::PrimeGenerator;

pub struct AtkinSieve;

impl PrimeGenerator for AtkinSieve {
    fn get_name(&self) -> &str {
        "Atkin sieve"
    }

    fn get_primes(&self, limit: usize) -> Vec<usize> {
        let sieve: Vec<usize> = (1..limit).collect();
        let mut is_prime = vec![false; limit];
        is_prime[1] = true;
        is_prime[2] = true;
        is_prime[4] = true;
        let limit_sqrt = (limit as f32).sqrt();
        'outer: for x in 1..=(limit_sqrt as usize) {
            for y in 1..=(limit_sqrt as usize) {
                let s1 = 4 * (x * x) + (y * y);
                let s2 = 3 * (x * x) + (y * y);
                let s3 = 3 * (x * x) - (y * y);

                if s1 > limit && s2 > limit && (x > y && s3 > limit) {
                    continue 'outer;
                }

                if s1 <= limit && [1, 13, 17, 29, 37, 41, 49, 53].contains(&(s1 % 60)) {
                    is_prime[s1 - 1] = !is_prime[s1 - 1]
                }

                if s2 <= limit && [7, 19, 31, 43].contains(&(s2 % 60)) {
                    is_prime[s2 - 1] = !is_prime[s2 - 1]
                }

                if s3 <= limit && x > y && [11, 23, 47, 59].contains(&(s3 % 60)) {
                    is_prime[s3 - 1] = !is_prime[s3 - 1]
                }
            }
        }

        let mut res: Vec<usize> = Vec::new();

        for i in 1..limit {
            if is_prime[i - 1] {
                res.push(sieve[i - 1]);
                if i - 1 < limit_sqrt as usize {
                    is_prime[i * i - 1] = false;
                }
            }
        }
        res
    }

    fn new() -> Self {
        Self {}
    }
}

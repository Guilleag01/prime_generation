use crate::prime_generators::PrimeGenerator;

pub struct WilsonTheorem;

impl PrimeGenerator for WilsonTheorem {
    fn get_name(&self) -> &str {
        "Wilson Theorem"
    }

    fn get_primes(&self, limit: usize) -> Vec<usize> {
        let mut primes = Vec::new();
        let mut last_prime = 1;
        let mut x = 1;

        while last_prime <= limit {
            last_prime = prime_n(x);
            primes.push(last_prime);
            x += 1;
        }

        primes
    }

    fn new() -> Self {
        Self {}
    }
}

fn factorial(x: f64) -> f64 {
    let mut a = 1.0;
    for i in 2..=x as usize {
        a *= i as f64;
    }
    a
}

fn prime_n(n: usize) -> usize {
    let mut total = 1.0;
    let dos: usize = 2;
    let upr_lmt: usize = dos.pow(n as u32);
    for i in 1..=upr_lmt {
        let mut a = 0.0;
        for j in 1..=i {
            a += (((factorial((j - 1) as f64) + 1.0) / j as f64 * std::f64::consts::PI).cos())
                .powf(2.0)
                .floor();
        }

        total += (n as f64 / a).powf(1.0 / n as f64).floor();
    }
    total as usize
}

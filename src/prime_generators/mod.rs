pub mod atkin_sieve;
pub mod dynamic;
pub mod erastosthenes_sieve;
pub mod one_line;
pub mod sundaram_sieve;

// use std::marker::Sized;

pub trait PrimeGenerator {
    fn new() -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn get_primes(&self, limit: usize) -> Vec<usize>;
}

// impl Sized for PrimeGenerator {}

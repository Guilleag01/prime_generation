use crate::prime_generators::PrimeGenerator;

pub struct ErastosthenesSieve;

impl PrimeGenerator for ErastosthenesSieve {
    fn get_name(&self) -> &str {
        "Erastothenes sieve"
    }

    fn get_primes(&self, limit: usize) -> Vec<usize> {
        let mut list: Vec<usize> = (2..limit).collect();
        let mut p = 2;
        let mut cont = true;
        'outer: while cont {
            let mut i = 2;
            while p * i - 2 < list.len() {
                list[p * i - 2] = 0;
                i += 1;
            }
            for j in &list[(p - 1)..] {
                if j != &0 {
                    p = *j;
                    continue 'outer;
                }
            }
            cont = false
        }

        list.into_iter().filter(|x| *x != 0).collect()
    }

    fn new() -> Self {
        Self {}
    }
}

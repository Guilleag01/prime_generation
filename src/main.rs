use std::{cmp::Ordering, time::Instant};

use prime_generation::prime_generators::{
    atkin_sieve::AtkinSieve, dynamic::Dynamic, erastosthenes_sieve::ErastosthenesSieve,
    one_line::OneLine, sundaram_sieve::SundaramSieve, PrimeGenerator,
};

fn main() {
    let limit = 1000000;

    println!("Starting with limit: {}\n", limit);

    let mut algorithms: [(Box<dyn PrimeGenerator>, f32); 5] = [
        (Box::new(AtkinSieve::new()), 0.0),
        (Box::new(Dynamic::new()), 0.0),
        (Box::new(ErastosthenesSieve::new()), 0.0),
        (Box::new(OneLine::new()), 0.0),
        (Box::new(SundaramSieve::new()), 0.0),
    ];

    for a in &mut algorithms {
        print!("Running {}...", a.0.get_name());
        let t0 = Instant::now();
        a.0.get_primes(limit);
        a.1 = t0.elapsed().as_secs_f32();
        println!("Done");
    }

    algorithms.sort_unstable_by(|x, y| {
        if x.1 > y.1 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    // println!("{:?}", algorithms[2].1);

    println!("\nResults with limit {}", limit);
    for a in algorithms {
        let mut text = String::new();
        text.push_str(a.0.get_name());
        // println!("{}", 5.0 - a.0.get_name().len() as f32 / 4.0);
        for _i in 0..(20 - a.0.get_name().len()) {
            text.push(' ');
        }
        text.push_str(a.1.to_string().as_str());
        println!("{}s", text);
    }
}

// 1.5
// Sundaram sieve      0.00000062
// 0.5
// Erastothenes sieve  0.00000072
// 3
// One line            0.00000148
// 3.25
// Dynamic             0.00000185
// 2.25
// Atkin sieve         0.00000366

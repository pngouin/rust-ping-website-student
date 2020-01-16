extern crate stopwatch;

use stopwatch::Stopwatch;

pub fn ping_domain(domain: &str, iterations: i64) -> i64 {
    let mut accumulator = 0;
    for _ in 0..iterations {
        accumulator += chrono(domain);
    }

    return accumulator / iterations;
}

pub fn chrono(domain: &str) -> i64 {
    let sw = Stopwatch::start_new();
    let _ = reqwest::blocking::get(domain);

    return sw.elapsed_ms();
}

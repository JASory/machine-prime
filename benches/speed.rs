use machine_prime::{is_prime, is_prime_wc};

const fn pi(x: u64) -> u64 {
    let mut inc = 0u64;
    let mut picount = 0u64;
    loop {
        inc += 1;
        if inc == x {
            return picount;
        }
        if is_prime(inc) {
            picount += 1;
        }
    }
}

const fn prng(mut x: u64) -> u64{
    x ^= x.wrapping_shr(12);
    x ^= x.wrapping_shl(25);
    x ^= x.wrapping_shr(27);
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

// Benchmarking the average in the strongest interval
// Real implementation will likely be faster

fn bench_average() {

    const DELTA: u64 = 100_000_000;
    const MAX: u64 = u64::MAX;

/*    
    const DELTA: u128 = 100_000_000;
    const MAX: u128 = u128::MAX;
  */  
    let start = std::time::Instant::now();
    let mut count = 0;
    for i in MAX - DELTA..MAX {
        if is_prime(i) {
            count += 1;
        }
    }
    let stop = start.elapsed();
    println!("Finished in t: {:?}", stop);
    println!(
        "{} sequential integers evaluated per second, finding {} primes",
        ((DELTA as u64) / stop.as_millis() as u64) * 1000,
        count
    );
    assert_eq!(count,2253052u64)
}

fn bench_rand(){
    const DELTA: u64 = 100_000_000;
    let mut integer : u64 = 0xAAAAAAAAAAAAAAAA;
    
    let start = std::time::Instant::now();
    let mut count = 0;
    for _ in 0..DELTA{
        integer = prng(integer);
        if is_prime(integer) {
            count += 1;
        }
    }
    let stop = start.elapsed();
    println!("Finished in t: {:?}", stop);
    println!(
        "{} pseudorandom integers evaluated per second, finding {} primes",
        ((DELTA as u64) / stop.as_millis() as u64) * 1000,
        count
    );
    assert_eq!(count,2307365u64)
}

// Benchmarking against the strongest case, real world will be faster
fn bench_worst() {
    const ITERATIONS: u64 = 10_000_000;

    const VALUE: u64 = u64::MAX - 58;

    const FLAG: bool = is_prime_wc(VALUE);
    let start = std::time::Instant::now();
    let mut count: u64 = 0;
    for _ in 0..ITERATIONS {
        if is_prime_wc(VALUE) {
            count += 1
        }
    }
    let stop = start.elapsed();
    println!("Finished in t: {:?}", stop);
    println!(
        "{} integers evaluated per second",
        (ITERATIONS / stop.as_millis() as u64) * 1000
    );
    println!("\"{} is prime\" is a {} statement", VALUE, FLAG);
    assert_eq!(count, ITERATIONS)
}

fn main() {
    
    let start = std::time::Instant::now();
    const PRIME_10K: u64 = pi(10_000);
    let stop = start.elapsed();
    println!(
        "{} primes under 10,000 calculated at compile-time {:?}",
        PRIME_10K, stop
    );
    
    bench_average();
    bench_rand();
    bench_worst();
}

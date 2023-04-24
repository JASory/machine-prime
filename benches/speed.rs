use machine_prime::is_prime_wc;
use machine_prime::is_prime_ac;

// Benchmarking the average in the strongest interval
// Real implementation will likely be faster 
fn bench_average(){

   const DELTA : u64 = 100_000_000;
   
   let start = std::time::Instant::now();
   let mut count = 0;
   for i in (u64::MAX-DELTA)..u64::MAX{
      if is_prime_ac(i){
        count+=1
      }
   }
   let stop = start.elapsed();
   println!("Finished in t: {:?}", stop);
   println!("{} integers evaluated per second, finding {} primes",(DELTA/stop.as_millis() as u64)*1000, count);
}

// Bencharking against the strongest case, real world will be faster
fn bench_worst(){
  const ITERATIONS : u64 = 10_000_000;
   
   let start = std::time::Instant::now();
   let mut count : u64 = 0;
   for _ in 0..ITERATIONS{
      if is_prime_wc(u64::MAX-58){
        count+=1
      }
   }
   let stop = start.elapsed();
   println!("Finished in t: {:?}", stop);
   println!("{} integers evaluated per second",(ITERATIONS/stop.as_millis() as u64)*1000);
   assert_eq!(count,ITERATIONS)
}

fn main(){
  bench_average();
  bench_worst();
}

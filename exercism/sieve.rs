#![feature(step_by)]
#![feature(test)]
extern crate test;

pub fn primes_up_to(n : usize) -> Vec<u32> {
    let mut is_prime = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for j in (4..n+1).step_by(2) {
        is_prime[j] = false;
    };
    let sq = (n as f32).sqrt() as usize;
    for i in (3..sq+1).step_by(2) {
        if is_prime[i] {
            for j in (i*i..n+1).step_by(i){
                is_prime[j] = false;
            }
        }
    };
    is_prime.iter().zip((0..n+1)).filter_map(
        |(&p, i)| if p {Some(i as u32)} else {None}).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_prime_1e6(b: &mut Bencher) {
        b.iter(|| primes_up_to(1e6 as usize));
    }

    #[bench]
    fn bench_prime_1e7(b: &mut Bencher) {
        b.iter(|| primes_up_to(1e7 as usize));
    }
}

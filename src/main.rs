use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    // We'll find all primes up to this number (inclusive)
    let n = 1_000_000;

    let start = Instant::now();
    let primes = find_primes(n);
    let elapsed = start.elapsed().as_micros();

    println!("found {} primes", primes.len()); // For n = 1_000_000, should be 78_498
    println!("in {elapsed} microseconds");
}

fn find_primes(n: u32) -> Vec<u32> {
    if n < 2 {
        return Vec::new();
    }

    let mut candidates = vec![true; n as usize + 1];

    candidates[0] = false;
    candidates[1] = false;

    let max = (n as f64).sqrt() as usize;
    let mut i = 2;

    while i <= max {
        if candidates[i] {
            let mut j = i * i;

            while j <= n as usize {
                candidates[j] = false;
                j += i;
            }
        }

        i += 1;
    }

    candidates
        .into_iter()
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(i, _)| i as u32)
        .collect()
}

fn _find_primes_deque(n: u32) -> Vec<u32> {
    let mut candidates: VecDeque<u32> = VecDeque::with_capacity(n as usize / 2);

    // Start candidates list at 3, and take only odd numbers
    let mut i = 3;
    while i <= n {
        candidates.push_back(i);
        i += 2;
    }

    // We have 2 as an initial freebie in our list of primes
    let mut primes: Vec<u32> = vec![2];

    while !candidates.is_empty() {
        // "Pop" the first item from the candidates list; that's our next prime
        let next = candidates.pop_front().unwrap();
        primes.push(next);

        // Early out; all remaining candidates will be prime
        if next * next > n {
            primes.append(&mut candidates.drain(..).collect::<Vec<u32>>());
            break;
        }

        // Otherwise, retain only candidates that are not multiples of `next`
        candidates.retain(|x| x % next != 0);
    }

    primes
}

fn _find_primes_vec(n: u32) -> Vec<u32> {
    let mut candidates: Vec<u32> = Vec::with_capacity(n as usize / 2);

    let mut i = 3;
    while i <= n {
        candidates.push(i);
        i += 2;
    }

    let mut primes: Vec<u32> = vec![2];

    while !candidates.is_empty() {
        let next = candidates.remove(0);
        primes.push(next);

        if next * next > n {
            primes.append(&mut candidates);
            break;
        }

        candidates.retain(|x| x % next != 0);
    }

    primes
}

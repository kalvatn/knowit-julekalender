use std::time::Instant;

use primes::is_prime;

fn number_contains_digit(n: &u32, d: &u32) -> bool {
  if n == d {
    return true;
  }
  let mut m: u32 = *n;
  while m > 0 {
    if m % 10 == *d {
      return true;
    }
    m = m / 10;
  }
  return false;
}

fn closest_lower_prime(n: &u32) -> u32 {
  let mut m = *n;
  while m > 2 {
    if is_prime(m as u64) {
      return m;
    }
    m -= 1;
  }
  m
}

fn solve(packages: &u32) -> u32 {
  let mut delivered = 0;
  let mut i = 0;
  while &i < packages {
    if number_contains_digit(&i, &7) {
      i += closest_lower_prime(&i);
    } else {
      delivered += 1;
    }
    i += 1;
  }
  delivered
}

fn main() {
  let input = 5433000;
  let time = Instant::now();
  let solution = solve(&input);
  println!("solution {:?} {:?}", solution, time.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_solve() {
    assert_eq!(solve(&10), 7);
    assert_eq!(solve(&20), 9);
    assert_eq!(solve(&10000), 32);
    assert_eq!(solve(&5433000), 69);
  }
}

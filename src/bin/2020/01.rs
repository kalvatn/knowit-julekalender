use std::time::Instant;

use common::io;
use std::collections::HashSet;

fn solve(input: &str) -> u32 {
  let numbers: HashSet<u32> = input
    .split(",")
    .map(|line| line.parse::<u32>().unwrap())
    .collect();
  (1u32..100001).find(|i| !numbers.contains(&i)).unwrap()
}

fn read_input() -> String {
  io::read_input(
    "https://julekalender-backend.knowit.no/challenges/1/attachments/numbers.txt",
    "2020-01",
  )
}

fn main() {
  let input = read_input();

  let p1_timer = Instant::now();
  println!(
    "solution {} {}µs",
    solve(&input),
    p1_timer.elapsed().as_micros()
  );
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_solve() {
    assert_eq!(solve(&read_input()), 81273);
  }
}

#![allow(unused_variables, unused_imports)]

use std::time::Instant;

use common::io;
use std::collections::HashSet;

fn read_input() -> String {
  io::read_input("", "2020-05")
}

fn solve(input: &str) -> u32 {
  0
}

fn main() {
  let input = read_input();
  println!("{:?}", input);
  let time = Instant::now();
  let solution = solve(&input);
  println!("solution {:?} {:?}", solution, time.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_solve() {
    assert_eq!(solve(&read_input()), 0);
  }
}

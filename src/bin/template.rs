#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens
)]

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use common::io;

fn read_input() -> String {
  io::read_input("", "2020-DD")
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

  const TEST_INPUT: &str = "";

  #[test]
  fn test_solve() {
    assert_eq!(solve(TEST_INPUT), 0);
    // assert_eq!(solve(&read_input()), 0);
  }
}

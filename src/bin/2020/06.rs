#![allow(unused_variables, unused_imports)]

use std::time::Instant;

use common::io;
use std::collections::HashSet;

fn read_input() -> String {
  io::read_input(
    "https://julekalender-backend.knowit.no/challenges/6/attachments/godteri.txt",
    "2020-06",
  )
}

fn solve(input: &str, elves: u32) -> (u32, u32) {
  let mut packages: Vec<u32> = input
    .split(",")
    .map(|s| s.parse::<u32>().unwrap())
    .collect();
  packages.sort();
  packages.reverse();
  let mut opened = 0;
  let mut items = 0;
  let mut solution = (0, 0);
  for i in packages {
    opened += 1;
    items += i;
    if items % elves == 0 {
      solution = (items / elves, opened);
    }
  }
  solution
}

fn main() {
  let input = read_input();
  let time = Instant::now();
  let solution = solve(&input, 127);
  println!("solution {:?} {:?}", solution, time.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &str = "14,10,14,15,14,13,13,13,15,11";

  #[test]
  fn test_solve() {
    assert_eq!(solve(TEST_INPUT, 9), (8, 5));
    // assert_eq!(solve(&read_input(),127), 0);
  }
}

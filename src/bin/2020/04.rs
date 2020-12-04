#![allow(unused_variables, unused_imports)]

use std::time::Instant;

use common::io;
use std::collections::HashSet;

fn read_input() -> String {
  io::read_input(
    "https://julekalender-backend.knowit.no/challenges/4/attachments/leveringsliste.txt",
    "2020-04",
  )
}

fn parse_input(input: &str) -> (u64, u64, u64, u64) {
  let mut milk = 0;
  let mut eggs = 0;
  let mut flour = 0;
  let mut sugar = 0;
  for x in input.lines() {
    for y in x.split(",") {
      let ingredient: Vec<&str> = y.split(":").map(|s| s.trim()).collect();
      let count = ingredient[1].parse::<u64>().unwrap();
      match ingredient[0] {
        "melk" => milk += count,
        "egg" => eggs += count,
        "mel" => flour += count,
        "sukker" => sugar += count,
        _ => unreachable!("impossiburu"),
      }
    }
  }
  return (milk, eggs, flour, sugar);
}

fn solve(input: &str) -> u64 {
  let (milk, eggs, flour, sugar) = parse_input(input);
  *vec![milk / 3, flour / 3, eggs, sugar / 2]
    .iter()
    .min()
    .unwrap()
}

fn main() {
  let input = read_input();
  let time = Instant::now();
  let solution = solve(&input);
  println!("solution {:?} {:?}", solution, time.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &str = "sukker: 24, mel: 20, egg: 17
sukker: 25, mel: 15, egg: 17
sukker: 17, melk: 14
sukker: 17, melk: 18
sukker: 25, melk: 24, egg: 10";

  #[test]
  fn test_solve() {
    assert_eq!(solve(TEST_INPUT), 11);
    assert_eq!(solve(&read_input()), 1458014);
  }
}

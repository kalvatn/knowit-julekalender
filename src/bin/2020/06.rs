use std::time::Instant;

use itertools::Itertools;

use common::io;

fn read_input() -> String {
  io::read_input(
    "https://julekalender-backend.knowit.no/challenges/6/attachments/godteri.txt",
    "2020-06",
  )
}

fn solve(input: &str, elves: u32) -> (u32, u32) {
  input
    .split(",")
    .map(|s| s.parse::<u32>().unwrap())
    .sorted()
    .rev()
    .scan(0u32, |acc, n| {
      *acc = *acc + n;
      Some(*acc)
    })
    .enumerate()
    .collect::<Vec<(usize, u32)>>()
    .into_iter()
    .rfind(|(_i, n)| n % elves == 0)
    .map(|(i, v)| (v / elves, i as u32 + 1))
    .unwrap()
  // let packages = input
  //   .split(",")
  //   .map(|s| s.parse::<u32>().unwrap())
  //   .sorted()
  //   .rev()
  //   .collect::<Vec<u32>>();
  // let mut opened = 0;
  // let mut items = 0;
  // let mut solution = (0, 0);
  // for i in packages {
  //   opened += 1;
  //   items += i;
  //   if items % elves == 0 {
  //     solution = (items / elves, opened);
  //   }
  // }
  // solution
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
    assert_eq!(solve(&read_input(), 127), (975, 9886));
  }
}

use std::time::Instant;

use common::io;

enum Direction {
  LEFT,
  RIGHT,
  UP,
  DOWN,
}

impl From<char> for Direction {
  fn from(c: char) -> Direction {
    match c {
      'V' => Direction::LEFT,
      'H' => Direction::RIGHT,
      'O' => Direction::UP,
      'N' => Direction::DOWN,
      _ => panic!("invalid direction"),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn mv(&mut self, dir: &Direction) -> &mut Point {
    match dir {
      Direction::UP => self.y -= 1,
      Direction::DOWN => self.y += 1,
      Direction::LEFT => self.x -= 1,
      Direction::RIGHT => self.x += 1,
    };
    self
  }
}
fn read_input() -> String {
  io::read_input(
    "https://julekalender-backend.knowit.no/challenges/5/attachments/rute.txt",
    "2020-05",
  )
}

fn parse_input(input: &str) -> Vec<Direction> {
  input.chars().map(|c| Direction::from(c)).collect()
}

fn solve(input: &str) -> u32 {
  let mut pos = Point { x: 0, y: 0 };
  let mut area = 0;
  let directions = parse_input(input);
  for d in directions {
    let prev = pos;
    pos.mv(&d);
    area += (prev.x * pos.y) - (prev.y * pos.x);
  }
  (area.abs() / 2) as u32
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

  #[test]
  fn test_solve() {
    assert_eq!(solve("HHOOVVNN"), 4);
    assert_eq!(solve("HHHHHHOOOOVVNNNVVOVVNN"), 14);
    assert_eq!(solve(&read_input()), 118000);
  }
}

use std::collections::HashSet;
use std::time::Instant;

use itertools::Itertools;

use common::io;

fn load_test_input() -> String {
  io::read_input("", "2020-07-test")
}

fn read_input() -> String {
  io::read_input(
    "https://julekalender-backend.knowit.no/challenges/7/attachments/forest.txt",
    "2020-07",
  )
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
  input
    .lines()
    .map(|l| {
      l.chars()
        .map(|c| match c {
          '#' => 1,
          _ => 0,
        })
        .collect()
    })
    .collect()
}

fn solve(input: &str) -> u32 {
  let grid = parse_input(input);

  let mut count_symmetric = 0;

  let bottom_row = grid.len() - 1;
  let first_branch_row = grid.len() - 3;
  let trunk_indices: Vec<usize> = grid[bottom_row]
    .iter()
    .enumerate()
    .filter(|x| x.1 == &1u8)
    .map(|x| x.0)
    .collect();
  for i in 0..trunk_indices.len() {
    let middle = trunk_indices[i];

    if grid[bottom_row - 1][middle] != 1 {
      continue;
    }

    let last_branch_index = (0..first_branch_row)
      .rev()
      .flat_map(|y| {
        grid[y]
          .iter()
          .enumerate()
          .dropping(middle)
          .take_while(|x| x.1 == &1u8)
          .last()
      })
      .map(|o| o.0)
      .max()
      .unwrap();
    let width = (last_branch_index - trunk_indices[i]) * 2 + 1;
    let start_x = last_branch_index - (width - 1);

    let mut y = first_branch_row;
    let mut symmetric = true;
    let width_first_branch = grid[y]
      .iter()
      .dropping(start_x)
      .take(width)
      .filter(|x| x == &&1u8)
      .count();
    let mut last_width = width;

    let mut top_found = false;
    let mut branch = 0;
    while y >= 1 || !top_found {
      // y = y -1;
      let current_row = &grid[y]
        .iter()
        .dropping(start_x)
        .take(width)
        .cloned()
        .collect::<Vec<u8>>();
      let next_row = &grid[y - 1]
        .iter()
        .dropping(start_x)
        .take(width)
        .cloned()
        .collect::<Vec<u8>>();
      let branch_width = current_row.iter().filter(|n| **n == 1u8).count();
      // println!("branch {} width {}, current {:?}, next {:?} ", branch, branch_width, current_row, next_row);
      if !is_symmetric(current_row) {
        symmetric = false;
        break;
      }
      if branch_width == 3 {
        top_found = next_row.iter().filter(|n| **n == 1u8).count() == 1;
        if !is_symmetric(next_row) {
          symmetric = false;
          break;
        }
        if top_found {
          break;
        } else {
          branch = 0;
          last_width = width;
        }
      } else {
        branch += 1;
        last_width = branch_width;
      }
      y = y - 1;
    }

    if symmetric {
      count_symmetric += 1;
    }
    println!();
  }
  count_symmetric
}

fn is_symmetric(vec: &Vec<u8>) -> bool {
  let size = vec.len();
  if size == 1 {
    return true;
  }
  if size % 2 == 0 {
    return false;
  }

  let mid = (size / 2);
  for i in 1..size - mid {
    if vec[mid - i] != vec[mid + i] {
      return false;
    }
  }

  return true;
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
  fn test_is_symmetric() {
    assert!(is_symmetric(&vec![1]));
    assert!(is_symmetric(&vec![0, 1, 0]));
    assert!(is_symmetric(&vec![0, 0, 1, 0, 0]));
    assert!(is_symmetric(&vec![1, 0, 1, 0, 1]));
    assert!(is_symmetric(&vec![1, 1, 1, 1, 1]));
    assert!(is_symmetric(&vec![1, 1, 1, 0, 0, 0, 1, 1, 1]));

    assert!(!is_symmetric(&vec![0, 0, 1, 0]));
    assert!(!is_symmetric(&vec![1, 1, 0]));
    assert!(!is_symmetric(&vec![0, 1, 1]));
    assert!(!is_symmetric(&vec![0, 0, 1, 1]));
    assert!(!is_symmetric(&vec![1, 1, 1, 0, 1, 1, 1, 1]));
  }

  #[test]
  fn test_solve() {
    assert_eq!(solve(&load_test_input()), 2);
    assert_eq!(solve(&read_input()), 5534);
  }
}

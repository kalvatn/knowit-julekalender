#![allow(unused_variables, unused_imports)]

use std::collections::HashSet;
use std::time::Instant;

use futures::TryFutureExt;
use itertools::Itertools;

use common::io;

fn read_grid_input() -> String {
  io::read_input("https://gist.githubusercontent.com/knowitkodekalender/d277d4f01a9fe10f7c1d92e2d17f1b31/raw/49da54e4372a83f4fc11d7137f19fc8b4c58bda6/matrix.txt", "2020-03-grid")
}

fn read_wordlist_input() -> String {
  io::read_input("https://gist.githubusercontent.com/knowitkodekalender/9e1ba20cd879b0c6d7af4ccfe8a87a19/raw/b19ae9548a33a825e2275d0283986070b9b7a126/wordlist.txt", "2020-03-wordlist")
}

fn rotate(grid: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
  let size = grid.len();
  let layer_count = size / 2;

  let mut matrix = grid.clone();

  for layer in 0..layer_count {
    let first = layer;
    let last = size - first - 1;

    for element in first..last {
      let offset = element - first;

      let top = matrix[first][element];
      let right_side = matrix[element][last];
      let bottom = matrix[last][last - offset];
      let left_side = matrix[last - offset][first];

      matrix[first][element] = left_side;
      matrix[element][last] = top;
      matrix[last][last - offset] = right_side;
      matrix[last - offset][first] = bottom;
    }
  }
  return matrix;
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
  input
    .lines()
    .map(|line| line.chars().into_iter().collect())
    .collect()
}

fn parse_wordlist(input: &str) -> Vec<String> {
  input.lines().map(String::from).collect()
}

fn solve(grid_input: &str, wordlist_input: &str) -> Vec<String> {
  let mut grid = parse_grid(grid_input);
  let wordlist = parse_wordlist(wordlist_input);
  let max_word_len = wordlist.iter().map(|w| w.len()).max().unwrap();
  let min_word_len = wordlist.iter().map(|w| w.len()).min().unwrap();
  // println!("{:?}", grid);
  // println!("{:?}", wordlist);
  // let mut groups:HashSet<Vec<(usize, usize)>> = HashSet::new();
  let size = grid.len();
  let mut strings: HashSet<String> = HashSet::new();
  for i in 0..4 {
    println!("i {:?}", i);
    for s in grid.iter().map(|row| row.iter().join("")) {
      strings.insert(s);
    }

    for group_size in min_word_len..max_word_len + 1 {
      for i in 0..size - (group_size) + 1 {
        for j in 0..size - (group_size) + 1 {
          let mut s = String::new();
          for k in 0..group_size {
            s.push(grid[j + k][i + k])
          }
          strings.insert(s);
        }
      }
    }
    // groups.iter().map(|g| {
    //   g.iter().map(|g| {
    //     grid[g.0][g.1]
    //   }).join("")
    // }).for_each(|s| {
    //   strings.insert(s);
    // });
    // println!("{:?}", groups);

    // println!("{:?}", s);
    // println!();
    grid = rotate(&mut grid);
  }
  println!("{:?}", strings.len());

  println!("removing found");
  let mut matches: HashSet<String> = HashSet::new();
  for s in &strings {
    for word in &wordlist {
      if s.contains(word) {
        matches.insert(word.to_string());
      }
    }
  }
  let mut vec1 = wordlist.clone();
  vec1.retain(|w| !matches.contains(w));
  vec1.sort();
  vec1
}

fn main() {
  let time = Instant::now();
  let grid_input = &read_grid_input();
  let wordlist_input = &read_wordlist_input();
  let solution = solve(grid_input, wordlist_input);
  println!("solution {:?} {:?}", solution, time.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_GRID: &str = "vlzzrkytiempkxg
wkuwuuniimpuzka
ufrazcavumtagod
ooscwzmvscdngwe
lskokdozvxvecer
povfkarkkmgoovf
vlirgaldqisatsg
pvknfgayzgqkcnn
iekozvnabdyapva
zgllegiizobkyjl
lgukatmaltamzba
lvnrvdizullcvsx
oscponrepvyatzy
rbhovtkpfljkihq
wjssiksnnergnal";
  //     const TEST_GRID: &str = "abc
  // def
  // ghi";
  const TEST_WORDLIST: &str = "kakao\nkriminalroman\nkvikklunch\nkylling\nlangfredag\nlangrennski\npalmesøndag\npåskeegg\nsmågodt";

  #[test]
  fn test_solve() {
    assert_eq!(
      solve(TEST_GRID, TEST_WORDLIST),
      vec!["palmesøndag", "påskeegg", "smågodt"]
    );
  }
}

use std::collections::{HashMap, HashSet};

use crate::vec::MyVec;

pub fn process(input: &str) -> usize {
  let grid: MyVec<MyVec<char>> = input
    .lines()
    .map(|l| l.chars().collect::<Vec<_>>().into())
    .collect::<Vec<_>>()
    .into();

  let rows = grid.len() as isize;
  let cols = grid[0].len() as isize;

  // directions for a 90-degree turn
  let directions = HashMap::from([
    ((-1, 0), (0, 1)),
    ((0, 1), (1, 0)),
    ((1, 0), (0, -1)),
    ((0, -1), (-1, 0)),
  ]);

  let mut direction: (isize, isize) = (-1, 0);

  // get guard position
  let mut guard_position = grid
    .iter()
    .enumerate()
    .filter_map(|(i, row)| {
      row
        .iter()
        .position(|c| c == &'^')
        .map(|pos| (i as isize, pos as isize))
    })
    .next()
    .unwrap();

  let mut visited = HashSet::new();
  // loop over matrix to fetch visited positions
  loop {
    visited.insert(guard_position);

    if guard_position.0 + direction.0 < 0
      || guard_position.0 + direction.0 >= rows
      || guard_position.1 + direction.1 < 0
      || guard_position.1 + direction.1 >= cols
    {
      break;
    }

    // rotate 90 degrees
    if grid[guard_position.0 + direction.0][guard_position.1 + direction.1] == '#' {
      direction = *directions.get(&direction).unwrap();
    } else {
      guard_position.0 += direction.0;
      guard_position.1 += direction.1;
    }
  }

  visited.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    assert_eq!(41, process(input));
  }
}

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::vec::MyVec;

// directions for a 90-degree turn
static DIRECTIONS: LazyLock<HashMap<(isize, isize), (isize, isize)>> = LazyLock::new(|| {
  HashMap::from([
    ((-1, 0), (0, 1)),
    ((0, 1), (1, 0)),
    ((1, 0), (0, -1)),
    ((0, -1), (-1, 0)),
  ])
});

pub fn process(input: &str) -> usize {
  let mut grid: MyVec<MyVec<char>> = input
    .lines()
    .map(|l| l.chars().collect::<Vec<_>>().into())
    .collect::<Vec<_>>()
    .into();

  // get guard position
  let guard_position = grid
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

  let rows = grid.len() as isize;
  let cols = grid[0].len() as isize;
  let mut count = 0_usize;

  for i in 0..rows {
    for j in 0..cols {
      if grid[i][j] != '.' {
        continue;
      }

      grid[i][j] = '#';

      if let Some(()) = check_loop(&grid, guard_position) {
        count += 1;
      }

      grid[i][j] = '.';
    }
  }

  count
}

fn check_loop(grid: &MyVec<MyVec<char>>, guard_position: (isize, isize)) -> Option<()> {
  let mut guard_position = guard_position;

  let rows = grid.len() as isize;
  let cols = grid[0].len() as isize;

  let mut direction: (isize, isize) = (-1, 0);

  let mut visited = HashSet::new();

  // loop over matrix to fetch visited positions
  loop {
    visited.insert((guard_position, direction));

    if guard_position.0 + direction.0 < 0
      || guard_position.0 + direction.0 >= rows
      || guard_position.1 + direction.1 < 0
      || guard_position.1 + direction.1 >= cols
    {
      return None;
    }

    // rotate 90 degrees
    if grid[guard_position.0 + direction.0][guard_position.1 + direction.1] == '#' {
      direction = *DIRECTIONS.get(&direction).unwrap();
    } else {
      guard_position.0 += direction.0;
      guard_position.1 += direction.1;
    }

    if visited.contains(&(guard_position, direction)) {
      return Some(());
    }
  }
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

    assert_eq!(6, process(input));
  }
}

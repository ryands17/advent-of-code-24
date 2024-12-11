use std::collections::VecDeque;

use utils::vec::MyVec;

pub fn process(input: &str) -> usize {
  let grid: MyVec<MyVec<u32>> = input
    .lines()
    .map(|l| {
      l.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into()
    })
    .collect::<Vec<_>>()
    .into();

  let trailheads = grid
    .iter()
    .enumerate()
    .flat_map(|(row, line)| {
      line
        .iter()
        .enumerate()
        .filter_map(|(col, digit)| {
          if *digit == 0 {
            return Some((row, col));
          }

          None
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  trailheads
    .iter()
    .map(|trailhead| fetch_trails(&grid, *trailhead))
    .sum()
}

fn fetch_trails(grid: &MyVec<MyVec<u32>>, start_position: (usize, usize)) -> usize {
  let mut bfs: VecDeque<(usize, usize)> = VecDeque::new();
  // let mut traversed = HashSet::new();

  let row_len = grid.len();
  let col_len = grid[0].len();

  let mut total = 0_usize;

  bfs.push_back(start_position);

  while !bfs.is_empty() {
    let cd = bfs.pop_front().unwrap();
    let row = cd.0 as isize;
    let col = cd.1 as isize;

    for (dr, dc) in [
      (row, col - 1),
      (row, col + 1),
      (row - 1, col),
      (row + 1, col),
    ] {
      // out of bounds
      if dr < 0 || dr >= row_len || dc < 0 || dc >= col_len {
        continue;
      }

      // is not a trail
      if grid[dr][dc] != grid[row][col] + 1 {
        continue;
      }

      if grid[dr][dc] == 9 {
        total += 1;
      } else {
        bfs.push_back((dr as usize, dc as usize));
      }
    }
  }

  total
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    assert_eq!(81, process(input));
  }
}

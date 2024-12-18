use std::collections::{HashSet, VecDeque};

use utils::vec::MyVec;

type Position = (isize, isize);

pub fn process(input: &str, edge: isize, valid_point: usize) -> (isize, isize) {
  let positions = input
    .lines()
    .map(|l| {
      let mut sp = l.split(',');
      let col = sp.next().unwrap().parse::<isize>().unwrap();
      let row = sp.next().unwrap().parse::<isize>().unwrap();

      (row, col)
    })
    .collect::<Vec<_>>();

  // because we know that up until the previous, all states are valid
  for bytes in valid_point + 1..positions.len() {
    if !has_valid_path(&positions, edge, bytes) {
      let (row, col) = positions[bytes - 1];
      return (col, row);
    }
  }

  (0, 0)
}

fn has_valid_path(positions: &[(isize, isize)], edge: isize, bytes: usize) -> bool {
  let mut grid: MyVec<MyVec<char>> = (0..edge + 1)
    .map(|_| (0..edge + 1).map(|_| '.').collect::<Vec<_>>().into())
    .collect::<Vec<_>>()
    .into();

  positions.iter().take(bytes).for_each(|(row, col)| {
    grid[*row][*col] = '#';
  });

  let mut q: VecDeque<Position> = VecDeque::from([(0, 0)]);
  let mut visited: HashSet<Position> = HashSet::from([(0, 0)]);

  while !q.is_empty() {
    let (row, col) = q.pop_front().unwrap();

    for (r, c) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let dr = row + r;
      let dc = col + c;

      if !is_valid_position((dr, dc), edge) {
        continue;
      }

      if grid[dr][dc] == '#' {
        continue;
      }

      if dr == edge && dc == edge {
        return true;
      }

      if visited.contains(&(dr, dc)) {
        continue;
      }

      visited.insert((dr, dc));
      q.push_back((dr, dc));
    }
  }

  false
}

fn is_valid_position(positon: Position, len: isize) -> bool {
  let row = positon.0;
  let col = positon.1;

  row >= 0 && row <= len && col >= 0 && col <= len
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    assert_eq!((6, 1), process(input, 6, 12));
  }
}

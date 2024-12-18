use std::collections::{HashSet, VecDeque};

use utils::vec::MyVec;

type Position = (isize, isize);

type Distance = usize;

pub fn process(input: &str, edge: isize, bytes: usize) -> usize {
  let mut grid: MyVec<MyVec<char>> = (0..edge + 1)
    .map(|_| (0..edge + 1).map(|_| '.').collect::<Vec<_>>().into())
    .collect::<Vec<_>>()
    .into();

  input.lines().take(bytes).for_each(|line| {
    let mut sp = line.split(',');
    let col = sp.next().unwrap().parse::<isize>().unwrap();
    let row = sp.next().unwrap().parse::<isize>().unwrap();

    grid[row][col] = '#';
  });

  let mut q: VecDeque<(Position, Distance)> = VecDeque::from([((0, 0), 0)]);
  let mut visited: HashSet<Position> = HashSet::from([(0, 0)]);

  while !q.is_empty() {
    let ((row, col), distance) = q.pop_front().unwrap();

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
        return distance + 1;
      }

      if visited.contains(&(dr, dc)) {
        continue;
      }

      visited.insert((dr, dc));
      q.push_back(((dr, dc), distance + 1));
    }
  }

  0
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

    assert_eq!(22, process(input, 6, 12));
  }
}

use std::collections::{HashMap, HashSet, VecDeque};

type Visited = HashSet<(usize, usize)>;
type Perimeters = HashMap<(usize, usize), Vec<(usize, usize)>>;

pub fn process(input: &str) -> usize {
  let grid: Vec<Vec<char>> = input
    .lines()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut visited: Visited = HashSet::new();
  let mut garden_plots: Perimeters = HashMap::new();

  grid.iter().enumerate().for_each(|(r, row)| {
    row.iter().enumerate().for_each(|(c, ch)| {
      if !visited.contains(&(r, c)) {
        garden_plots.insert((r, c), find_garden_plot(&grid, *ch, (r, c), &mut visited));
      }
    });
  });

  garden_plots
    .values()
    .map(|plots| plots.len() * get_perimeter(&grid, plots))
    .sum()
}

fn get_perimeter(grid: &[Vec<char>], plots: &[(usize, usize)]) -> usize {
  // check all 4 sides of the cell
  plots
    .iter()
    .map(|cell| {
      let mut count = 0_usize;

      for (dr, dc) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let (r, c) = (cell.0 as isize + dr, cell.1 as isize + dc);

        if r < 0 || c < 0 {
          count += 1;
          continue;
        }

        let r = r as usize;
        let c = c as usize;

        if r >= grid.len() || c >= grid[0].len() {
          count += 1;
          continue;
        }

        if grid[r][c] != grid[cell.0][cell.1] {
          count += 1;
        }
      }

      count
    })
    .sum()
}

fn find_garden_plot(
  grid: &[Vec<char>],
  ch: char,
  pos: (usize, usize),
  visited: &mut Visited,
) -> Vec<(usize, usize)> {
  let mut coordinates: Vec<(usize, usize)> = Vec::new();
  coordinates.push(pos);

  let mut bfs: VecDeque<(usize, usize)> = VecDeque::new();
  bfs.push_back(pos);
  visited.insert(pos);

  while !bfs.is_empty() {
    let node = bfs.pop_front().unwrap();

    for (dr, dc) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
      let (r, c) = (node.0 as isize + dr, node.1 as isize + dc);

      if r < 0 || c < 0 {
        continue;
      }

      let r = r as usize;
      let c = c as usize;

      if r >= grid.len() || c >= grid[0].len() {
        continue;
      }

      if grid[r][c] == ch && !visited.contains(&(r, c)) {
        // add to visited
        visited.insert((r, c));

        // add to perimeter vector
        coordinates.push((r, c));

        // add to queue
        bfs.push_back((r, c));
      }
    }
  }

  coordinates
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input_1() {
    let input = r"AAAA
BBCD
BBCC
EEEC";

    assert_eq!(140, process(input));
  }

  #[test]
  fn sample_input_2() {
    let input = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    assert_eq!(772, process(input));
  }

  #[test]
  fn sample_input_3() {
    let input = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    assert_eq!(1930, process(input));
  }
}

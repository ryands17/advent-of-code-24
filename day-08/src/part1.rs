use std::collections::{HashMap, HashSet};

pub fn process(input: &str) -> usize {
  let grid = input
    .lines()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let row_len = grid.len() as isize;
  let col_len = grid[0].len() as isize;

  let mut nodemap: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

  grid.iter().enumerate().for_each(|(r, row)| {
    row.iter().enumerate().for_each(|(c, col)| {
      if *col != '.' {
        let list = nodemap.entry(*col).or_default();
        list.push((r, c));
      }
    });
  });

  let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
  nodemap.values().for_each(|coords| {
    for r in 0..coords.len() {
      for c in (r + 1)..coords.len() {
        // add top mirror (r.0 - (c.0 - r.0), r.1 - (c.1 - r.1))
        let top_antinode = (
          2 * coords[r].0 as isize - coords[c].0 as isize,
          2 * coords[r].1 as isize - coords[c].1 as isize,
        );

        antinodes.insert(top_antinode);

        // add bottom mirror (c.0 + (c.0 - r.0), c.1 + (c.1 - r.1))
        let bottom_antinode = (
          2 * coords[c].0 as isize - coords[r].0 as isize,
          2 * coords[c].1 as isize - coords[r].1 as isize,
        );

        antinodes.insert(bottom_antinode);
      }
    }
  });

  antinodes
    .iter()
    .filter(|val| 0 <= val.0 && val.0 < row_len && 0 <= val.1 && val.1 < col_len)
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    assert_eq!(14, process(input));
  }
}

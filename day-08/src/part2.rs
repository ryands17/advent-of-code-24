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
        // check for top mirrors
        let mut r_tmp = coords[r].0 as isize;
        let mut c_tmp = coords[r].1 as isize;

        let r_delta = coords[c].0 as isize - coords[r].0 as isize;
        let c_delta = coords[c].1 as isize - coords[r].1 as isize;

        // add all top antinodes (r.0 - (c.0 - r.0), r.1 - (c.1 - r.1))
        while 0 <= r_tmp && r_tmp < row_len && 0 <= c_tmp && c_tmp < col_len {
          antinodes.insert((r_tmp, c_tmp));

          r_tmp -= r_delta;
          c_tmp -= c_delta;
        }

        // check for bottom mirrors
        let mut r_tmp = coords[c].0 as isize;
        let mut c_tmp = coords[c].1 as isize;

        let r_delta = coords[c].0 as isize - coords[r].0 as isize;
        let c_delta = coords[c].1 as isize - coords[r].1 as isize;

        // add all bottom antinodes (c.0 + (c.0 - r.0), c.1 + (c.1 - r.1))
        while 0 <= r_tmp && r_tmp < row_len && 0 <= c_tmp && c_tmp < col_len {
          antinodes.insert((r_tmp, c_tmp));

          r_tmp += r_delta;
          c_tmp += c_delta;
        }
      }
    }
  });

  antinodes.len()
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

    assert_eq!(34, process(input));
  }
}

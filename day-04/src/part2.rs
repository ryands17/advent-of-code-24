pub fn process(input: &str) -> usize {
  let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut count = 0_usize;

  for row in 1..(matrix.len() - 1) {
    for col in 1..(matrix[row].len() - 1) {
      // ignore cells that are not 'A'
      if matrix[row][col] != 'A' {
        continue;
      }

      // M.M
      // .A.
      // S.S
      if matrix[row - 1][col - 1] == 'M'
        && matrix[row - 1][col + 1] == 'M'
        && matrix[row + 1][col - 1] == 'S'
        && matrix[row + 1][col + 1] == 'S'
      {
        count += 1;
      }

      // S.S
      // .A.
      // M.M
      if matrix[row - 1][col - 1] == 'S'
        && matrix[row - 1][col + 1] == 'S'
        && matrix[row + 1][col - 1] == 'M'
        && matrix[row + 1][col + 1] == 'M'
      {
        count += 1;
      }

      // M.S
      // .A.
      // M.S
      if matrix[row - 1][col - 1] == 'M'
        && matrix[row - 1][col + 1] == 'S'
        && matrix[row + 1][col - 1] == 'M'
        && matrix[row + 1][col + 1] == 'S'
      {
        count += 1;
      }

      // S.M
      // .A.
      // S.M
      if matrix[row - 1][col - 1] == 'S'
        && matrix[row - 1][col + 1] == 'M'
        && matrix[row + 1][col - 1] == 'S'
        && matrix[row + 1][col + 1] == 'M'
      {
        count += 1;
      }
    }
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    assert_eq!(9, process(input));
  }
}

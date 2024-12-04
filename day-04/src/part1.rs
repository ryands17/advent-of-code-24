pub fn process(input: &str) -> usize {
  let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut count = 0_usize;

  for row in 0..matrix.len() {
    for col in 0..matrix[row].len() {
      // ignore cells that are not 'X'
      if matrix[row][col] != 'X' {
        continue;
      }

      // check top
      if row >= 3
        && matrix[row - 1][col] == 'M'
        && matrix[row - 2][col] == 'A'
        && matrix[row - 3][col] == 'S'
      {
        count += 1;
      }

      // check bottom
      if row < matrix.len() - 3
        && matrix[row + 1][col] == 'M'
        && matrix[row + 2][col] == 'A'
        && matrix[row + 3][col] == 'S'
      {
        count += 1;
      }

      // check left
      if col >= 3
        && matrix[row][col - 1] == 'M'
        && matrix[row][col - 2] == 'A'
        && matrix[row][col - 3] == 'S'
      {
        count += 1;
      }

      // check right
      if col < matrix[row].len() - 3
        && matrix[row][col + 1] == 'M'
        && matrix[row][col + 2] == 'A'
        && matrix[row][col + 3] == 'S'
      {
        count += 1;
      }

      // check top left
      if row >= 3
        && col >= 3
        && matrix[row - 1][col - 1] == 'M'
        && matrix[row - 2][col - 2] == 'A'
        && matrix[row - 3][col - 3] == 'S'
      {
        count += 1;
      }

      // check top right
      if row >= 3
        && col < matrix[row].len() - 3
        && matrix[row - 1][col + 1] == 'M'
        && matrix[row - 2][col + 2] == 'A'
        && matrix[row - 3][col + 3] == 'S'
      {
        count += 1;
      }

      // check bottom left
      if row < matrix.len() - 3
        && col >= 3
        && matrix[row + 1][col - 1] == 'M'
        && matrix[row + 2][col - 2] == 'A'
        && matrix[row + 3][col - 3] == 'S'
      {
        count += 1;
      }

      // check bottom right
      if row < matrix.len() - 3
        && col < matrix[row].len() - 3
        && matrix[row + 1][col + 1] == 'M'
        && matrix[row + 2][col + 2] == 'A'
        && matrix[row + 3][col + 3] == 'S'
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
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    assert_eq!(18, process(input));
  }
}

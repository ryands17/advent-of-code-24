use crate::helpers::calculate_checksum;

pub fn process(input: &str) -> usize {
  let mut blocks: Vec<isize> = vec![];
  let mut block_id = 0;

  input.chars().enumerate().for_each(|(i, c)| {
    let count = c.to_digit(10).unwrap();

    if i % 2 == 0 {
      blocks.extend((0..count).map(|_| block_id));
      block_id += 1;
    } else {
      blocks.extend((0..count).map(|_| -1));
    }
  });

  // 2-pointer approach
  let mut left = 0;
  let mut right = blocks.len() - 1;

  while left < right {
    // check for empty space
    if blocks[left] == -1 && blocks[right] != -1 {
      blocks.swap(left, right);

      left += 1;
      right -= 1;

      continue;
    }

    if blocks[left] != -1 {
      left += 1;
    }

    if blocks[right] == -1 {
      right -= 1;
    }
  }

  calculate_checksum(&blocks)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"2333133121414131402";

    assert_eq!(1928, process(input));
  }
}

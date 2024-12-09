use crate::helpers::calculate_checksum;

pub fn process(input: &str) -> usize {
  let mut blocks: Vec<isize> = vec![];
  let mut file_ids: Vec<(isize, isize)> = vec![];
  let mut spaces: Vec<(isize, isize)> = vec![];

  let mut block_id = 0;

  input.chars().enumerate().for_each(|(i, c)| {
    let count = c.to_digit(10).unwrap();

    if i % 2 == 0 {
      file_ids.push((blocks.len() as isize, count as isize));

      blocks.extend((0..count).map(|_| block_id));
      block_id += 1;
    } else {
      if count != 0 {
        spaces.push((blocks.len() as isize, count as isize));
      }

      blocks.extend((0..count).map(|_| -1));
    }
  });
  file_ids.reverse();

  for meta in &file_ids {
    for (space_idx, space) in spaces.iter().enumerate() {
      // if space isn't big enough to fit the file
      if meta.1 > space.1 {
        continue;
      }

      // if space is to the right of the file
      if space.0 > meta.0 {
        break;
      }

      // fit the file
      let mut index = space.0;
      let mut count = space.1;
      let mut i = 0;

      // swap the memory file
      while i < meta.1 {
        blocks.swap((meta.0 + i) as usize, (space.0 + i) as usize);

        // incrment the filled space index and decrement the count
        index += 1;
        count -= 1;

        i += 1;
      }

      if count == 0 {
        spaces.remove(space_idx);
      } else {
        spaces[space_idx] = (index, count);
      }

      break;
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

    assert_eq!(2858, process(input));
  }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
  Increasing,
  Decreasing,
}

pub fn process(input: &str) -> usize {
  input
    .lines()
    .filter_map(|line| {
      let nums = line
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

      for i in 0..nums.len() {
        let level = exclude_index(&nums, i);

        match check_validity(&level) {
          None => (),
          Some(_) => return Some(()),
        }
      }

      None
    })
    .count()
}

pub fn check_validity(nums: &[usize]) -> Option<()> {
  let direction: Direction = {
    if nums[0] < nums[1] {
      Direction::Increasing
    } else {
      Direction::Decreasing
    }
  };

  for i in 1..nums.len() {
    // check for direction mismatch
    if nums[i] > nums[i - 1] && direction == Direction::Decreasing {
      return None;
    }

    if nums[i] < nums[i - 1] && direction == Direction::Increasing {
      return None;
    }

    // check for offset mismatch
    let diff = nums[i].abs_diff(nums[i - 1]);
    if !(1..=3).contains(&diff) {
      return None;
    }
  }

  Some(())
}

fn exclude_index(vec: &[usize], index: usize) -> Vec<usize> {
  let mut result = vec![];

  if index > 0 {
    // Elements before the index
    result.extend_from_slice(&vec[0..index]);
  }
  if index + 1 < vec.len() {
    // Elements after the index
    result.extend_from_slice(&vec[index + 1..]);
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    assert_eq!(4, process(input));
  }
}

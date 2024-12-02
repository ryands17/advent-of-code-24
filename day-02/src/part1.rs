#[derive(Debug, Clone, PartialEq)]
enum Direction {
  Increasing,
  Decreasing,
}

pub fn process(input: &str) -> usize {
  input
    .lines()
    .map(|line| {
      let nums = line
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

      check_validity(&nums)
    })
    .filter(|x| x.is_some())
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

    assert_eq!(2, process(input));
  }
}

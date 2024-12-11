pub fn process(input: &str) -> usize {
  let mut stones = input
    .split(' ')
    .map(|n| n.parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  for _ in 0..25 {
    stones = blink(stones);
  }

  stones.len()
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
  let mut new_numbers = Vec::new();

  for n in stones {
    let n_str = n.to_string();

    if n == 0 {
      new_numbers.push(1);
    } else if n_str.len() % 2 == 0 {
      let (first, second) = n_str.split_at(n_str.len() / 2);

      new_numbers.push(first.parse::<usize>().unwrap());
      new_numbers.push(second.parse::<usize>().unwrap());
    } else {
      new_numbers.push(n * 2024);
    }
  }

  new_numbers
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"125 17";

    assert_eq!(55312, process(input));
  }
}

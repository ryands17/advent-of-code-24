use std::collections::HashMap;

pub fn process(input: &str) -> usize {
  let stones = input
    .split(' ')
    .map(|n| n.parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

  stones
    .into_iter()
    .map(|stone| blink(stone, 75, &mut cache))
    .sum()
}

fn blink(stone: usize, steps: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
  if steps == 0 {
    return 1;
  }

  match cache.get(&(stone, steps)) {
    Some(val) => *val,
    None => {
      let mut result = 0_usize;
      let stone_str = stone.to_string();

      if stone == 0 {
        result = blink(1, steps - 1, cache);
      } else if stone_str.len() % 2 == 0 {
        let (first, second) = stone_str.split_at(stone_str.len() / 2);

        result += blink(first.parse::<usize>().unwrap(), steps - 1, cache)
          + blink(second.parse::<usize>().unwrap(), steps - 1, cache);
      } else {
        result += blink(stone * 2024, steps - 1, cache);
      }

      cache.insert((stone, steps), result);

      *cache.get(&(stone, steps)).unwrap()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"125 17";

    assert_eq!(65601038650482, process(input));
  }
}

use std::collections::HashMap;

type Cache = HashMap<(usize, usize), usize>;

pub fn process(input: &str) -> usize {
  let stones = input
    .split(' ')
    .map(|n| n.parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  let mut cache: Cache = HashMap::new();

  stones
    .into_iter()
    .map(|stone| blink(stone, 25, &mut cache))
    .sum()
}

fn blink(stone: usize, steps: usize, cache: &mut Cache) -> usize {
  if steps == 0 {
    return 1;
  }

  match cache.get(&(stone, steps)) {
    Some(val) => *val,
    None => {
      let stone_str = stone.to_string();

      if stone == 0 {
        let result = blink(1, steps - 1, cache);
        cache.insert((stone, steps), result);
        return result;
      }

      if stone_str.len() % 2 == 0 {
        let (first, second) = stone_str.split_at(stone_str.len() / 2);

        let result = blink(first.parse::<usize>().unwrap(), steps - 1, cache)
          + blink(second.parse::<usize>().unwrap(), steps - 1, cache);

        cache.insert((stone, steps), result);
        return result;
      } else {
        let result = blink(stone * 2024, steps - 1, cache);
        cache.insert((stone, steps), result);
        return result;
      }
    }
  }
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

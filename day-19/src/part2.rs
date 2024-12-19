use std::collections::HashMap;

type Cache = HashMap<String, usize>;

pub fn process(input: &str) -> usize {
  let mut sp = input.split("\n\n");

  let patterns = sp.next().unwrap().split(", ").collect::<Vec<_>>();
  let designs = sp.next().unwrap().lines().collect::<Vec<_>>();

  let mut cache: Cache = HashMap::new();

  designs
    .into_iter()
    .map(|design| count_possible_designs(design, &patterns, &mut cache))
    .sum()
}

fn count_possible_designs(design: &str, patterns: &[&str], cache: &mut Cache) -> usize {
  if let Some(val) = cache.get(design) {
    return *val;
  }

  if design.is_empty() {
    return 1;
  }

  let mut count = 0;

  for i in 1..=design.len() {
    if patterns.contains(&&design[..i]) {
      count += count_possible_designs(&design[i..], patterns, cache);
    }
  }

  cache.insert(design.to_string(), count);

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    assert_eq!(16, process(input));
  }
}

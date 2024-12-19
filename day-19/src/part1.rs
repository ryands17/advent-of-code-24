use std::collections::HashMap;

type Cache = HashMap<String, bool>;

pub fn process(input: &str) -> usize {
  let mut sp = input.split("\n\n");

  let patterns = sp.next().unwrap().split(", ").collect::<Vec<_>>();
  let designs = sp.next().unwrap().lines().collect::<Vec<_>>();

  let mut cache: Cache = HashMap::new();

  designs
    .into_iter()
    .filter_map(|design| is_design_possible(design, &patterns, &mut cache))
    .count()
}

fn is_design_possible(design: &str, patterns: &[&str], cache: &mut Cache) -> Option<bool> {
  if let Some(val) = cache.get(design) {
    return match val {
      true => Some(true),
      false => None,
    };
  }

  if design.is_empty() {
    return Some(true);
  }

  for i in 1..=design.len() {
    if patterns.contains(&&design[..i])
      && is_design_possible(&design[i..], patterns, cache).is_some()
    {
      cache.insert(design.to_string(), true);
      return Some(true);
    }
  }

  cache.insert(design.to_string(), false);

  None
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

    assert_eq!(6, process(input));
  }
}

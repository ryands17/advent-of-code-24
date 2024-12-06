use std::sync::LazyLock;

use regex::Regex;

static RE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex"));

pub fn process(input: &str) -> usize {
  RE.captures_iter(input)
    .map(|cap| {
      let a = cap[1].parse::<usize>().unwrap();
      let b = cap[2].parse::<usize>().unwrap();

      a * b
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    assert_eq!(161, process(input));
  }
}

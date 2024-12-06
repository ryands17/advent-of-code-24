use std::sync::LazyLock;

use regex::Regex;

static RE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").expect("Invalid regex")
});

pub fn process(input: &str) -> usize {
  let mut valid_mul = true;

  RE.captures_iter(input)
    .filter_map(|cap| {
      let item = cap.get(0).unwrap().as_str();

      if item == "don't()" {
        valid_mul = false;
        return None;
      } else if item == "do()" {
        valid_mul = true;
        return None;
      }

      if valid_mul {
        let a = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let b = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();

        return Some(a * b);
      }

      None
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    assert_eq!(48, process(input));
  }
}

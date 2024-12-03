use regex::Regex;

pub fn process(input: &str) -> usize {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

  re.captures_iter(input)
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

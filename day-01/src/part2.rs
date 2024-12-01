pub fn process(input: &str) -> usize {
  let mut v1: Vec<usize> = vec![];
  let mut v2: Vec<usize> = vec![];

  input.lines().for_each(|line| {
    let mut nums = line.split(' ');

    v1.push(nums.next().unwrap().parse::<usize>().unwrap());
    v2.push(nums.last().unwrap().parse::<usize>().unwrap());
  });

  v1.iter()
    .map(|item| v2.iter().filter(|v| *v == item).count() * *item)
    .sum::<usize>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(31, process(input));
  }
}

pub fn process(input: &str) -> u32 {
  let mut v1 = vec![];
  let mut v2 = vec![];

  input.lines().for_each(|line| {
    let mut nums = line.split(' ');

    v1.push(nums.next().unwrap().parse::<u32>().unwrap());
    v2.push(nums.last().unwrap().parse::<u32>().unwrap());
  });

  v1.sort();
  v2.sort();

  v1.iter().zip(v2.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
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

    assert_eq!(11, process(input));
  }
}

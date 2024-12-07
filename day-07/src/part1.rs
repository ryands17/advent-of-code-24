enum Operation {
  Add,
  Multiply,
}

pub fn process(input: &str) -> usize {
  input
    .lines()
    .filter_map(|line| {
      let mut sp = line.split(':');
      let target = sp.next().unwrap().parse::<usize>().unwrap();

      let values = sp
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

      if is_target_match(target, &values, 0) {
        return Some(target);
      }

      None
    })
    .sum()
}

fn is_target_match(target: usize, list: &[usize], current: usize) -> bool {
  if list.is_empty() {
    return target == current;
  }

  if current > target {
    return false;
  }

  if is_target_match(
    target,
    &list[1..],
    operate(current, list[0], Operation::Add),
  ) {
    return true;
  }

  let current = if current == 0 { 1 } else { current };
  is_target_match(
    target,
    &list[1..],
    operate(current, list[0], Operation::Multiply),
  )
}

fn operate(n1: usize, n2: usize, op: Operation) -> usize {
  match op {
    Operation::Add => n1 + n2,
    Operation::Multiply => n1 * n2,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(3749, process(input));
  }
}

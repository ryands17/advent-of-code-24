use std::collections::HashMap;

pub fn process(input: &str) -> usize {
  let mut parts = input.split("\n\n");
  let mut mappings = HashMap::new();

  parts.next().unwrap().lines().for_each(|l| {
    let mut sp = l.split("|");

    let n1 = sp.next().unwrap().parse::<usize>().unwrap();
    let n2 = sp.next().unwrap().parse::<usize>().unwrap();

    mappings.insert((n1, n2), true);
    mappings.insert((n2, n1), false);
  });

  parts
    .next()
    .unwrap()
    .lines()
    .map(|l| {
      let mut vec = l
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

      // logic to check mappings
      let mut is_valid = true;

      'main: for i in 0..(vec.len() - 1) {
        for j in (i + 1)..vec.len() {
          match mappings.get(&(vec[i], vec[j])) {
            None => {
              is_valid = false;
              break 'main;
            }
            Some(false) => {
              is_valid = false;
              break 'main;
            }
            _ => {}
          }
        }
      }

      if !is_valid {
        vec.sort_by(|a, b| match mappings.get(&(*a, *b)) {
          Some(true) => std::cmp::Ordering::Less,
          Some(false) => std::cmp::Ordering::Greater,
          None => std::cmp::Ordering::Equal,
        });

        return vec[vec.len() / 2];
      }

      0
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    assert_eq!(123, process(input));
  }
}

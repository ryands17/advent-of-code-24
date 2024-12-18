use regex::Regex;

pub fn process(input: &str) -> String {
  let re = Regex::new(r"\d+").expect("Invalid regex");

  let mut matches = re.find_iter(input);

  let mut a = matches.next().unwrap().as_str().parse::<usize>().unwrap();
  let mut b = matches.next().unwrap().as_str().parse::<usize>().unwrap();
  let mut c = matches.next().unwrap().as_str().parse::<usize>().unwrap();

  let program = matches
    .filter_map(|m| m.as_str().parse::<usize>().ok())
    .collect::<Vec<_>>();

  let mut i = 0;
  let mut output: Vec<usize> = vec![];

  while i < program.len() - 1 {
    let combo = |op: usize| -> usize {
      match op {
        0..=3 => op,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand!"),
      }
    };

    let code = program[i];
    let op = program[i + 1];

    match code {
      // adv
      0 => a >>= combo(op),
      // bxl
      1 => b ^= op,
      // bst
      2 => b = combo(op) % 8,
      // jnz
      3 => {
        if a != 0 {
          i = op;
          continue;
        }
      }
      // bxc
      4 => b ^= c,
      // out
      5 => output.push(combo(op) % 8),
      // bdv
      6 => b = a >> combo(op),
      // cdv
      7 => c = a >> combo(op),
      _ => panic!("Invalid code!"),
    }

    i += 2;
  }

  output
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<_>>()
    .join(",")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input));
  }
}

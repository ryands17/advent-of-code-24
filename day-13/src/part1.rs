use regex::Regex;

pub fn process(input: &str) -> usize {
  let re = Regex::new(r"\d+").expect("Invalid regex");

  input
    .split("\n\n")
    .filter_map(|block| {
      let mut nums = re.find_iter(block);

      // find values for equations
      let ax = nums.next().unwrap().as_str().parse::<f64>().unwrap();
      let ay = nums.next().unwrap().as_str().parse::<f64>().unwrap();

      let bx = nums.next().unwrap().as_str().parse::<f64>().unwrap();
      let by = nums.next().unwrap().as_str().parse::<f64>().unwrap();

      let rx = nums.next().unwrap().as_str().parse::<f64>().unwrap();
      let ry = nums.next().unwrap().as_str().parse::<f64>().unwrap();

      // calculate equation using linear algebra with these two equations
      // x * ax + y * bx = rx
      // x * ay + y * by = ry
      // Solving for the above, we get
      // x = (rx * by - ry * bx) / (ax * by - ay * bx)
      // y = (rx - x * ax) / bx

      let x = (rx * by - ry * bx) / (ax * by - ay * bx);
      let y = (rx - x * ax) / bx;

      if x.fract() != 0.0 || y.fract() != 0.0 {
        return None;
      }

      let x: usize = x.trunc() as usize;
      let y: usize = y.trunc() as usize;

      if x >= 100 || y >= 100 {
        return None;
      }

      Some(x * 3 + y)
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    assert_eq!(480, process(input));
  }
}

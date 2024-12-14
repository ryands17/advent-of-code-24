use std::collections::HashSet;

use regex::Regex;

type Robot = ((isize, isize), (isize, isize));

pub fn process(input: &str, rows: isize, cols: isize) -> usize {
  let re = Regex::new(r"-?\d+").expect("Invalid regex");

  let mut seconds = 0_usize;

  let mut robots = input
    .lines()
    .map(|line| {
      let mut matches = re.find_iter(line);

      let rx = matches.next().unwrap().as_str().parse::<isize>().unwrap();
      let ry = matches.next().unwrap().as_str().parse::<isize>().unwrap();

      let vx = matches.next().unwrap().as_str().parse::<isize>().unwrap();
      let vy = matches.next().unwrap().as_str().parse::<isize>().unwrap();

      ((rx, ry), (vx, vy))
    })
    .collect::<Vec<_>>();

  // the concept of a christmas tree is to make sure that no robots are overlapping
  loop {
    // increment the robot steps
    seconds += 1;
    update_position(&mut robots, rows, cols);

    if !are_robots_overlapping(&robots) {
      break;
    }
  }

  seconds
}

fn update_position(robots: &mut [Robot], rows: isize, cols: isize) {
  for robot in robots {
    robot.0 .0 = (robot.0 .0 + robot.1 .0).rem_euclid(cols);
    robot.0 .1 = (robot.0 .1 + robot.1 .1).rem_euclid(rows);
  }
}

fn are_robots_overlapping(robots: &[Robot]) -> bool {
  let mut positions = HashSet::new();

  for robot in robots {
    if positions.contains(&robot.0) {
      return true;
    }

    positions.insert(robot.0);
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    assert_eq!(1, process(input, 7, 11));
  }
}

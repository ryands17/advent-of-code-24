use regex::Regex;

pub fn process(input: &str, rows: isize, cols: isize) -> usize {
  let re = Regex::new(r"-?\d+").expect("Invalid regex");

  let rows_middle = rows / 2;
  let cols_middle = cols / 2;

  let seconds = 100_isize;

  let mut top_left = 0_usize;
  let mut top_right = 0_usize;
  let mut bottom_left = 0_usize;
  let mut bottom_right = 0_usize;

  input
    .lines()
    .map(|line| {
      let mut matches = re.find_iter(line);

      let rx = matches.next().unwrap().as_str().parse::<isize>().unwrap();
      let ry = matches.next().unwrap().as_str().parse::<isize>().unwrap();

      let vx = matches.next().unwrap().as_str().parse::<isize>().unwrap();
      let vy = matches.next().unwrap().as_str().parse::<isize>().unwrap();

      (
        (rx + vx * seconds).rem_euclid(cols),
        (ry + vy * seconds).rem_euclid(rows),
      )
    })
    .for_each(|(x, y)| {
      if x == cols_middle || y == rows_middle {
        return;
      }

      // top
      if y < rows_middle {
        if x < cols_middle {
          top_left += 1;
        } else {
          top_right += 1;
        }
      }

      // bottom
      if y > rows_middle {
        if x < cols_middle {
          bottom_left += 1;
        } else {
          bottom_right += 1;
        }
      }
    });

  top_left * top_right * bottom_left * bottom_right
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

    assert_eq!(12, process(input, 7, 11));
  }
}

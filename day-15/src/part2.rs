use utils::vec::MyVec;

type Position = (isize, isize);

pub fn process(input: &str) -> usize {
  let mut sp = input.split("\n\n");

  let mut grid: MyVec<MyVec<char>> = sp
    .next()
    .unwrap()
    .lines()
    .map(|line| {
      let mut double = vec![];
      line.chars().for_each(|ch| match ch {
        '#' => {
          double.push('#');
          double.push('#');
        }
        'O' => {
          double.push('[');
          double.push(']');
        }
        '.' => {
          double.push('.');
          double.push('.');
        }
        _ => {
          double.push('@');
          double.push('.');
        }
      });

      double.into()
    })
    .collect::<Vec<_>>()
    .into();

  let moves = sp.next().unwrap().replace('\n', "");

  let mut r_pos: Position = {
    let mut row = 0_isize;
    let mut col = 0_isize;

    'main: for (i, r) in grid.iter().enumerate() {
      for (j, c) in r.iter().enumerate() {
        if c == &'@' {
          row = i as isize;
          col = j as isize;

          break 'main;
        }
      }
    }

    (row, col)
  };

  print_grid(&grid);

  for m in moves.chars() {
    println!("Move: {m}");
    let mut targets: Vec<Position> = Vec::new();
    targets.push(r_pos);

    let direction = get_direction(m);
    get_targets(&grid, r_pos, direction, &mut targets);

    if targets.iter().any(|(r, c)| grid[*r][*c] == '#') {
      continue;
    }

    let (dr, dc) = direction;
    let grid_copy: MyVec<MyVec<char>> = grid
      .iter()
      .map(|c| c.0.clone().into())
      .collect::<Vec<_>>()
      .into();

    println!("targets: {:?}", targets);
    // for the robot
    grid[r_pos.0][r_pos.1] = '.';
    r_pos = (r_pos.0 + dr, r_pos.1 + dc);
    grid[r_pos.0][r_pos.1] = '@';

    for (r, c) in targets.iter().skip(1) {
      grid[*r][*c] = '.';
    }

    for (r, c) in targets.iter().skip(1) {
      grid[r + dr][c + dc] = grid_copy[*r][*c];
    }

    print_grid(&grid);
  }

  let mut sum = 0_usize;
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == '[' {
        sum += 100 * i as usize + j as usize;
      }
    }
  }

  sum
}

fn get_targets(
  grid: &MyVec<MyVec<char>>,
  current: Position,
  direction: Position,
  targets: &mut Vec<Position>,
) {
  let (dr, dc) = (current.0 + direction.0, current.1 + direction.1);

  if grid[dr][dc] == '#' {
    targets.push((dr, dc));
    return;
  }

  if grid[dr][dc] == '[' {
    if !targets.contains(&(dr, dc)) {
      targets.push((dr, dc));
      get_targets(grid, (dr, dc), direction, targets);
    }

    if !targets.contains(&(dr, dc + 1)) {
      targets.push((dr, dc + 1));
      get_targets(grid, (dr, dc + 1), direction, targets);
    }
  }

  if grid[dr][dc] == ']' {
    if !targets.contains(&(dr, dc)) {
      targets.push((dr, dc));
      get_targets(grid, (dr, dc), direction, targets);
    }

    if !targets.contains(&(dr, dc - 1)) {
      targets.push((dr, dc - 1));
      get_targets(grid, (dr, dc - 1), direction, targets);
    }
  }
}

fn print_grid(grid: &MyVec<MyVec<char>>) {
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      print!("{} ", grid[i][j]);
    }
    println!("");
  }

  println!("\n\n");
}

fn get_direction(ch: char) -> Position {
  match ch {
    '^' => (-1, 0),
    '>' => (0, 1),
    'v' => (1, 0),
    _ => (0, -1),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input_1() {
    let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    assert_eq!(9021, process(input));
  }

  #[test]
  fn sample_input_2() {
    let input = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    assert_eq!(618, process(input));
  }
}

use utils::vec::MyVec;

type Position = (isize, isize);

pub fn process(input: &str) -> usize {
  let mut sp = input.split("\n\n");

  let mut grid: MyVec<MyVec<char>> = sp
    .next()
    .unwrap()
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>().into())
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
    let mut targets: Vec<Position> = Vec::new();
    targets.push(r_pos);

    let mut should_move = true;

    let (dr, dc) = get_direction(m);
    let (mut r, mut c) = (r_pos.0, r_pos.1);

    loop {
      r += dr;
      c += dc;

      match grid[r][c] {
        '#' => {
          should_move = false;
          break;
        }
        'O' => {
          targets.push((r, c));
        }
        _ => {
          break;
        }
      }
    }

    if !should_move {
      continue;
    }

    for (i, (r, c)) in targets.into_iter().enumerate() {
      if i == 0 {
        // for the robot
        grid[r_pos.0][r_pos.1] = '.';
        r_pos = (r_pos.0 + dr, r_pos.1 + dc);
        grid[r_pos.0][r_pos.1] = '@';
      } else {
        grid[r + dr][c + dc] = 'O';
      }
    }

    print_grid(&grid);
  }

  let mut sum = 0_usize;
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'O' {
        sum += 100 * i as usize + j as usize;
      }
    }
  }

  sum
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

    assert_eq!(10092, process(input));
  }

  #[test]
  fn sample_input_2() {
    let input = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    assert_eq!(2028, process(input));
  }
}

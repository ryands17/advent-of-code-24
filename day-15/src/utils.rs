use utils::vec::MyVec;

pub type Position = (isize, isize);

pub fn print_grid(grid: &MyVec<MyVec<char>>) {
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      print!("{} ", grid[i][j]);
    }
    println!();
  }

  println!("\n\n");
}

pub fn get_direction(ch: char) -> Position {
  match ch {
    '^' => (-1, 0),
    '>' => (0, 1),
    'v' => (1, 0),
    _ => (0, -1),
  }
}

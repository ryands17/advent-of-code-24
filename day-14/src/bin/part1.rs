use day_14::part1;

fn main() {
  let text = include_str!("../../input1.txt");
  let val = part1::process(text, 103, 101);

  println!("{val}");
}

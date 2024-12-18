use day_18::part1;

fn main() {
  let text = include_str!("../../input1.txt");
  let val = part1::process(text, 70, 1024);

  println!("{val}");
}

use day_14::part2;

fn main() {
  let input = include_str!("../../input2.txt");
  let result = part2::process(input, 103, 101);

  println!("{result}");
}

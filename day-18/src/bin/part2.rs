use day_18::part2;

fn main() {
  let input = include_str!("../../input2.txt");
  let result = part2::process(input, 70, 1024);

  println!("{:?}", result);
}

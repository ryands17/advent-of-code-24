pub fn calculate_checksum(blocks: &[isize]) -> usize {
  blocks
    .iter()
    .enumerate()
    .map(|(i, val)| if *val == -1 { 0 } else { i * *val as usize })
    .sum()
}

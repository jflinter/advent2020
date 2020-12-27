fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let ids: Vec<u32> = input.lines().map(|l| part05::seat_id(l)).collect();
  let missing = part05::find_missing(&ids).unwrap();
  let max = ids.iter().max().unwrap();
  println!("max seat id: {}, missing: {}", max, missing);
}

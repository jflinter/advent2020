fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let result = part06::yeses_per_input(&input);
  println!("result: {}", result);
}

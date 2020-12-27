fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  println!("possibilities: {}", part07::bag_count(&input, "shiny gold"));
}

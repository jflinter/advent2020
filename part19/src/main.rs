fn main() {
  let input = std::fs::read_to_string("part19/input.txt").unwrap();
  println!("{}", part19::num_matching(&input));
}

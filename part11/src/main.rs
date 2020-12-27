fn main() {
  let input = std::fs::read_to_string("part11/input.txt").unwrap();
  let boat = part11::Boat::parse(&input);
  println!("{}", boat);
  let res = boat.capacity();
  println!("{}", res)
}

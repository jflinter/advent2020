pub fn main() {
  let input = std::fs::read_to_string("part12/input.txt").unwrap();
  let actions = input.lines().map(|l| part12::Action::parse(l).unwrap());
  let mut boat = part12::Boat::new();
  for a in actions {
    boat = part12::Boat::apply(&boat, a);
  }
  println!("{:?}, {}", boat, boat.manhattan());
}

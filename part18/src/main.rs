use part18::Node;

fn main() {
  let input = std::fs::read_to_string("part18/input.txt").unwrap();
  let sum: usize = input.lines().map(|line| Node::parse(line).eval()).sum();
  println!("got {}", sum)
}

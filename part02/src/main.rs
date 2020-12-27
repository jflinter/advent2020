fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let count = input.lines().filter(|line| part02::valid_line(line)).count();
  println!("Count is {}", count);
}

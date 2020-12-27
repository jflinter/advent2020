fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let passports = part04::valid_passports(&input);
  println!("valid passports: {}", passports);
}

fn main() {
  let input = std::fs::read_to_string("part16/input.txt").unwrap();
  let input = part16::Input::parse(&input);
  let res = input.ticket_scanning_error_rate();
  println!("error rate: {}", res);
  println!("field mappings: {:?}", input.multiplied_departure_values());
}

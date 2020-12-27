fn main() {
  let input = std::fs::read_to_string("part09/input.txt").unwrap();
  let ints = input.lines().map(|l| l.parse().unwrap()).collect();
  let res = part09::first_nonsum(&ints, 25);
  let weak = part09::encryption_weakness(&ints, 25);
  println!("got {:?}", res);
  println!("got {:?}", weak);
}

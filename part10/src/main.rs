fn main() {
  let input = std::fs::read_to_string("part10/input.txt").unwrap();
  let ints = input.lines().map(|l| l.parse().unwrap()).collect();
  let res = part10::munge_jolts(&ints);
  let res2 = part10::num_orderings(&ints);
  println!("got {}, {}", res, res2)
}

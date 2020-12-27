fn main() {
  let input = std::fs::read_to_string("part08/input.txt").unwrap();
  let program = part08::Program::parse(&input).unwrap();
  let res = program.munge_and_run();
  println!("result: {:?}", res);
}

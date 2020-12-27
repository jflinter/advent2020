fn main() {
  let mut input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
  input.extend((input.len() + 1)..=1_000_000);
  println!("len: {}", input.len());
  let res = part23::crabby(input, 10000000);
  println!("res: {}", res.len());
}

use part03::Vector;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let slopes = vec![
    Vector(1, 1),
    Vector(3, 1),
    Vector(5, 1),
    Vector(7, 1),
    Vector(1, 2),
  ];
  println!("hit {} trees", part03::trees_encountered(&input, &slopes[2]));
  println!("hit {} many trees", part03::many_trees_encountered(&input, slopes));
}

pub struct Vector (pub usize, pub usize);

pub fn many_trees_encountered(input: &str, slopes: Vec<Vector>) -> u32 {
  slopes.iter().map(|s| trees_encountered(input, s)).product()
}

pub fn trees_encountered(input: &str, v: &Vector) -> u32 {
  let mut point = (0, 0);
  let mut trees = 0;
  let lines: Vec<&str> = input.lines().collect();
  while point.1 < lines.len() {
    let line = lines.get(point.1).unwrap();
    let value = line.chars().nth(point.0).unwrap();
    if value == '#' {
      trees += 1;
    }
    point = ((point.0 + v.0) % line.len(), point.1 + v.1);
  }
  return trees;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trees_encountered() {
      let test = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
      assert_eq!(trees_encountered(test, &Vector(3, 1)), 7);
    }

    #[test]
    fn test_many_trees_encountered() {
      let test = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
      let slopes = vec![
        Vector(1, 1),
        Vector(3, 1),
        Vector(5, 1),
        Vector(7, 1),
        Vector(1, 2),
      ];
      assert_eq!(many_trees_encountered(test, slopes), 336);
    }
}


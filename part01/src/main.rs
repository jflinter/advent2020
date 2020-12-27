use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: HashSet<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let result = lines.iter().find_map(|x| {
      find(&lines, 2020 - *x).map(|pair| (*x, pair.0, pair.1))
    }).unwrap();    
    println!("product: {}", result.0 * result.1 * result.2);
}

struct IntPair(u32, u32);

fn find(lines: &HashSet<u32>, sum: u32) -> Option<IntPair> {
  lines.iter().find(|x| {
    *x < &sum && lines.contains(&(sum - *x))
  }).map(|x| IntPair(*x, sum - x))
}

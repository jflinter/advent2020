use regex::Regex;

pub fn valid_line(line: &str) -> bool {
  let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
  let caps = re.captures(line).unwrap();
  let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
  let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
  let c = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
  let password = caps.get(4).unwrap().as_str();
  // let occurrences = password.matches(c).count() as u32;
  // std::ops::Range { start: min, end: max+1 }.contains(&occurrences)
  // println!("{}, {}, {}, {}, {}", min, max, c, password, occurrences);
  let lhs = password.chars().nth(min - 1);
  let rhs = password.chars().nth(max - 1);
  // println!("{:?}, {:?}", lhs, rhs);
  if let (Some(l), Some(r)) = (lhs, rhs) {
    (l == c) ^ (r == c)
  } else {
    false
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_line() {
      assert!(valid_line("1-3 a: abcde"));
      assert!(!valid_line("1-3 b: cdefg"));
      assert!(!valid_line("2-9 c: ccccccccc"));
    }
}


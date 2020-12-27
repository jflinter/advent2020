pub fn encryption_weakness(input: &Vec<usize>, window_size: usize) -> Option<usize> {
  let nonsum = first_nonsum(input, window_size)?;
  let range = range_summing(input, nonsum)?;
  Some(range.iter().max()? + range.iter().min()?)
}

fn range_summing(input: &Vec<usize>, target: usize) -> Option<&[usize]> {
  (0..input.len())
    .into_iter()
    .find_map(|i| range_summing_starting_at(input, target, i))
}

fn range_summing_starting_at(
  input: &Vec<usize>,
  target: usize,
  starting_at: usize,
) -> Option<&[usize]> {
  let mut i = starting_at.clone();
  let mut sum = 0;
  while let Some(v) = input.get(i) {
    sum += v;
    if sum == target {
      return Some(&input[starting_at..=i]);
    } else if sum > target {
      return None;
    }
    i += 1;
  }
  None
}

pub fn first_nonsum(input: &Vec<usize>, window_size: usize) -> Option<usize> {
  let mut i = window_size.clone();
  while let Some(v) = input.get(i) {
    if !any_sums(&input[(i - window_size)..i], *v) {
      return Some(*v);
    }
    i += 1;
  }
  None
}

fn any_sums(values: &[usize], sum: usize) -> bool {
  values
    .iter()
    .filter(|i| **i < sum && 2 * **i != sum)
    .any(|f| values.contains(&(sum - f)))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<usize> {
    let x = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    x.lines().map(|l| l.parse().unwrap()).collect()
  }

  #[test]
  fn test_first_nonsum() {
    assert_eq!(first_nonsum(&input(), 5), Some(127));
  }

  #[test]
  fn test_range_summing() {
    let expected: &[usize] = &[15, 25, 47, 40];
    assert_eq!(range_summing_starting_at(&input(), 127, 2), Some(expected));
    assert_eq!(range_summing(&input(), 127), Some(expected));
  }

  #[test]
  fn test_encryption_weakness() {
    assert_eq!(encryption_weakness(&input(), 5), Some(62));
  }
}

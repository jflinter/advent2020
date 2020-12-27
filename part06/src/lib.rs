use std::collections::HashSet;

pub trait Reduce<T> {
    fn reduce<F>(self, f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T;
}

impl<T, I> Reduce<T> for I
where
    I: Iterator<Item = T>,
{
    #[inline]
    fn reduce<F>(mut self, f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T,
    {
        self.next().map(|first| self.fold(first, f))
    }
}

pub fn yeses_per_input(input: &str) -> u32 {
  input.split("\n\n").into_iter().map(yeses_per_group).sum()
}

pub fn yeses_per_group(input: &str) -> u32 {
  let lines = input.split_ascii_whitespace();
  lines.into_iter()
    .map(|l| l.chars().collect::<HashSet<_>>())
    .reduce(|a, b| a.intersection(&b).copied().collect())
    .unwrap()
    .len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yeses_per_group() {
      let input = "a\nb\nc";
      assert_eq!(0, yeses_per_group(input));

      let input = "a\na\na";
      assert_eq!(1, yeses_per_group(input));

      let input = "ab\nab\nab";
      assert_eq!(2, yeses_per_group(input));

      let input = "abc";
      assert_eq!(3, yeses_per_group(input));

      let input = "ab\nac";
      assert_eq!(1, yeses_per_group(input));

      let input = "a\na\na\na";
      assert_eq!(1, yeses_per_group(input));

      let input = "b";
      assert_eq!(1, yeses_per_group(input));
    }

    #[test]
    fn test_yeses_per_input() {
      let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
      assert_eq!(6, yeses_per_input(input));
    }
}


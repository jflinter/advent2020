pub fn seat_id(input: &str) -> u32 {
  let row = parse_val(&input[0..7], "F", "B");
  let seat = parse_val(&input[7..10], "L", "R");
  (row * 8) + seat
}

fn parse_val(input: &str, zero_val: &str, one_val: &str) -> u32 {
  let bin = input.replace(zero_val, "0").replace(one_val, "1");
  u32::from_str_radix(&bin, 2).unwrap()
}

pub fn find_missing(ids: &Vec<u32>) -> Option<u32> {
  let min = ids.iter().min()?;
  let max = ids.iter().max()?;
  let sum: u32 = ids.iter().sum();
  let len = max - min;
  let expected = (len + 1) * min + (len * (len + 1) / 2);
  Some(expected - sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_number() {
      let input = "FBFBBFFRLR";
      assert_eq!(seat_id(input), 357);
    }

    #[test]
    fn test_find_missing() {
      assert_eq!(7, find_missing(&vec![4, 5, 6, 8, 9]).unwrap());
    }
}


pub fn bus_times_minutes(depart_ts: usize, departures: &Vec<usize>) -> usize {
  let earliest = earliest_bus(depart_ts, departures);
  let time = minutes_until_bus(depart_ts, earliest);
  earliest * time
}

fn minutes_until_bus(depart_ts: usize, departure: usize) -> usize {
  departure - (depart_ts % departure)
}

fn earliest_bus(depart_ts: usize, departures: &Vec<usize>) -> usize {
  *departures
    .iter()
    .min_by_key(|d| minutes_until_bus(depart_ts, **d))
    .unwrap()
}

#[cfg(test)]
mod tests {
  use crate::earliest_bus;

  #[test]
  fn it_works() {
    assert_eq!(earliest_bus(939, &vec![7, 13, 59, 31, 19]), 59);
  }
}

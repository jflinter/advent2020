fn main() {
  let input = std::fs::read_to_string("part13/input.txt").unwrap();
  let lines: Vec<&str> = input.lines().collect();
  let ts: usize = lines[0].parse().unwrap();
  let departures: Vec<usize> = lines[1]
    .split(",")
    .map(|c| c.parse::<usize>())
    .filter_map(Result::ok)
    .collect();
  let res = part13::bus_times_minutes(ts, &departures);
  println!("{}", res);
}

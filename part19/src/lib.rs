extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
#[derive(Parser)]
#[grammar = "test.pest"]
// #[grammar = "input.pest"]
pub struct TestParser;

impl TestParser {
  pub fn matches(input: &str) -> bool {
    TestParser::parse(Rule::R0, input).is_ok()
  }
}

pub fn num_matching(input: &str) -> usize {
  let pieces = input.split("\n\n").collect::<Vec<_>>();
  let tests = pieces[1];
  tests
    .lines()
    .filter(|test| TestParser::matches(*test))
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let input =
      std::fs::read_to_string("/Users/jackflintermann/Developer/advent2020/part19/test.txt")
        .unwrap();
    assert_eq!(num_matching(&input), 3);
  }
}

use std::{collections::HashMap, ops::RangeInclusive};
use std::{collections::HashSet, iter};

#[derive(Debug, PartialEq, Clone)]
struct Rule {
  name: String,
  ranges: Vec<RangeInclusive<usize>>,
}

impl Rule {
  fn valid(&self, input: usize) -> bool {
    self.ranges.iter().any(|r| r.contains(&input))
  }
  fn parse(text: &str) -> Rule {
    let parts = text.split(": ").collect::<Vec<_>>();
    let name = parts[0];
    let ranges = parts[1]
      .split(" or ")
      .map(|p| {
        let range = p.split("-").collect::<Vec<_>>();
        range[0].parse().unwrap()..=range[1].parse().unwrap()
      })
      .collect();

    Rule {
      name: name.to_string(),
      ranges,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Input {
  rules: Vec<Rule>,
  your_ticket: Vec<usize>,
  nearby_tickets: Vec<Vec<usize>>,
}

impl Input {
  pub fn multiplied_departure_values(&self) -> usize {
    self
      .field_mappings()
      .iter()
      .filter_map(|(key, idx)| {
        if key.starts_with("departure") {
          Some(idx)
        } else {
          None
        }
      })
      .map(|idx| self.your_ticket[*idx])
      .product()
  }

  fn all_tickets(&self) -> impl Iterator<Item = &Vec<usize>> + '_ {
    self
      .nearby_tickets
      .iter()
      .filter(move |v| v.iter().all(|f| self.rules.iter().any(|r| r.valid(*f))))
      .chain(iter::once(&self.your_ticket))
  }
  pub fn field_mappings(&self) -> HashMap<String, usize> {
    // start with a vector of hashsets.
    // the index of the vector will be the rule
    // the hashset will contain the fields it is possible for it to contain.
    println!("initial state: {:?}", self);
    let mut possibilities = self
      .rules
      .iter()
      .map(|rule| {
        (0..self.rules.len())
          .filter(move |i| self.all_tickets().all(|ticket| rule.valid(ticket[*i])))
          .collect::<HashSet<_>>()
      })
      .collect::<Vec<_>>();
    println!("initial possibilities: {:?}", possibilities);
    loop {
      let new_possibilities = Input::constrain(&possibilities);
      if new_possibilities.eq(&possibilities) {
        break;
      } else {
        possibilities = new_possibilities;
      }
    }
    if possibilities.iter().all(|p| p.len() == 1) {
      let map: HashMap<_, _> = self
        .rules
        .iter()
        .zip(possibilities.iter())
        .map(|(r, s)| (r.name.to_string(), *s.iter().next().unwrap()))
        .collect();
      return map;
    } else {
      panic!("still need to relax more: {:?}", possibilities);
    }
  }

  fn constrain(possibilities: &Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let confirmed = possibilities
      .iter()
      .filter_map(|p| {
        if p.len() == 1 {
          Some(p.iter().collect::<Vec<_>>()[0])
        } else {
          None
        }
      })
      .collect::<HashSet<_>>();
    let p2 = possibilities.clone();
    let vals = p2
      .into_iter()
      .map(|p| {
        if p.len() == 1 {
          p
        } else {
          let filtered = p
            .into_iter()
            .filter_map(|val| -> Option<usize> {
              if confirmed.contains(&val) {
                None
              } else {
                Some(val)
              }
            })
            .collect();
          filtered
        }
      })
      .collect();
    vals
  }

  pub fn ticket_scanning_error_rate(&self) -> usize {
    self
      .nearby_tickets
      .iter()
      .map(|ticket| -> usize {
        ticket
          .iter()
          .filter(|v| !self.rules.iter().any(|r| r.valid(**v)))
          .sum()
      })
      .sum()
  }

  pub fn parse(text: &str) -> Input {
    let sections: Vec<_> = text.split("\n\n").collect();
    let rules_section = sections[0];
    let rules = rules_section.lines().map(|l| Rule::parse(l)).collect();

    let your_ticket_line = sections[1].lines().collect::<Vec<_>>()[1];
    let your_ticket = your_ticket_line
      .split(",")
      .map(|e| e.parse().unwrap())
      .collect();
    let nearby_tickets = sections[2]
      .lines()
      .skip(1)
      .map(|l| l.split(",").map(|e| e.parse().unwrap()).collect())
      .collect();

    Input {
      rules,
      your_ticket,
      nearby_tickets,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> &'static str {
    "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
  }
  #[test]
  fn it_works() {
    let input = Input::parse(input());
    println!("mappings: {:?}", input.field_mappings());
  }
}

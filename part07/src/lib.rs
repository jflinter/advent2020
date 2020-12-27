use std::collections::HashMap;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Dfs;
use regex::Regex;

#[derive(PartialEq, Debug)]
struct Rule {
  outer_bag: String,
  inner_bags: Vec<(u32, String)>,
}

pub fn bag_possibilities(input: &str, color: &str) -> u32 {
  let rules: Vec<_> = input.lines().map(parse).map(Option::unwrap).collect();
  let mut graph = DiGraphMap::new();
  let mut nodes = HashMap::new();
  for rule in rules.iter() {
    let node = graph.add_node(rule.outer_bag.as_str());
    nodes.insert(rule.outer_bag.as_str(), node);
  }
  for rule in rules.iter() {
    for (_, inner) in rule.inner_bags.iter() {
      let parent_node = nodes.get(rule.outer_bag.as_str()).unwrap();
      let child_node = nodes.get(inner.as_str()).unwrap();
      graph.add_edge(*child_node, *parent_node, 1);
    }
  }
  let start = nodes.get(color).unwrap();
  let mut dfs = Dfs::new(&graph, *start);
  let mut i = 0;
  while let Some(_) = dfs.next(&graph) {
    i += 1
  }
  i
}

pub fn bag_count(input: &str, color: &str) -> u32 {
  let rules: Vec<_> = input.lines().map(parse).map(Option::unwrap).collect();
  let mut graph = DiGraphMap::new();
  let mut nodes = HashMap::new();
  for rule in rules.iter() {
    let node = graph.add_node(rule.outer_bag.as_str());
    nodes.insert(rule.outer_bag.as_str(), node);
  }
  for rule in rules.iter() {
    for (weight, inner) in rule.inner_bags.iter() {
      let parent_node = nodes.get(rule.outer_bag.as_str()).unwrap();
      let child_node = nodes.get(inner.as_str()).unwrap();
      graph.add_edge(*parent_node, *child_node, weight);
    }
  }
  bags(&graph, nodes[color]) - 1
}

fn bags(graph: &DiGraphMap<&str, &u32>, start: &str) -> u32 {
  let mut count = 1;
  for (_source, target, &weight) in graph.edges(start) {
    count += weight * bags(graph, target);
  }
  count
}

fn parse(line: &str) -> Option<Rule> {
  let re_left = Regex::new(r"(.+) bags contain .*$").unwrap();
  let re_right = Regex::new(r"(\d+) (\D+) bag").unwrap();
  let outer_bag = re_left.captures(line)?.get(1)?.as_str().to_string();
  let inner_bags = re_right.captures_iter(line).map(|c| {
    (c[1].to_string().parse().unwrap(), c[2].to_string())
  }).collect();
  Some(Rule { outer_bag, inner_bags })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
      let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
      assert_eq!(
        parse(input),
        Some(Rule { outer_bag: "light red".to_string(), inner_bags: vec![(1, "bright white".to_string()), (2, "muted yellow".to_string())]})
      )
    }

    #[test]
    fn test_bag_count() {
      let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
      assert_eq!(126, bag_count(input, "shiny gold"));
    }
}


use std::collections::HashMap;

use itertools::Itertools;
use petgraph::graphmap::DiGraphMap;

pub fn num_orderings(input: &Vec<usize>) -> usize {
  let max = input.iter().max().unwrap() + 3;
  let extras = vec![0, max, max, max];
  let extras = extras.iter();
  let mut graph = DiGraphMap::new();
  for (a, b, c, d) in input
    .iter()
    .chain([0].iter())
    .chain(extras)
    .sorted()
    .tuple_windows::<(_, _, _, _)>()
  {
    graph.add_node(a);
    for &i in [b, c, d].iter() {
      if i != a && i - a <= 3 {
        graph.add_edge(a, i, 0);
      }
    }
  }
  let mut weights: HashMap<usize, usize> = HashMap::new();
  for i in input.iter().rev() {
    num_orderings_inner(&graph, *i, max, &mut weights);
  }
  num_orderings_inner(&graph, 0, max, &mut weights)
}

fn num_orderings_inner(
  graph: &DiGraphMap<&usize, i32>,
  src: usize,
  target: usize,
  weights: &mut HashMap<usize, usize>,
) -> usize {
  if src == target {
    return 1;
  }
  if let Some(x) = weights.get(&src) {
    return *x;
  }
  let res: usize = graph
    .neighbors(&src)
    .map(|n| num_orderings_inner(graph, *n, target, weights))
    .sum();
  weights.insert(src, res);
  res
}

pub fn munge_jolts(input: &Vec<usize>) -> usize {
  let (a, b): (Vec<usize>, Vec<usize>) = input
    .iter()
    .chain([0].iter())
    .sorted()
    .tuple_windows::<(_, _)>()
    .map(|(a, b)| b - a)
    .filter(|s| *s == 1 || *s == 3)
    .partition(|s| *s == 1);
  a.len() * (b.len() + 1) // add 1 for the device itself
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input(ex1: bool) -> Vec<usize> {
    let x = if ex1 {
      "16
10
15
5
1
11
7
19
6
12
4"
    } else {
      "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
    };
    x.lines().map(|l| l.parse().unwrap()).collect()
  }

  #[test]
  fn test_munge_jolts() {
    assert_eq!(munge_jolts(&input(true)), 35);
  }

  #[test]
  fn test_num_orderings() {
    assert_eq!(num_orderings(&input(true)), 8);
    assert_eq!(num_orderings(&input(false)), 19208);
  }
}

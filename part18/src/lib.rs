#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Add(Box<Node>, Box<Node>),
  Mul(Box<Node>, Box<Node>),
  Lit(usize),
}

trait IntoNode {
  fn into(self) -> Node;
}

impl Node {
  pub fn eval(&self) -> usize {
    match self {
      Node::Add(a, b) => a.eval() + b.eval(),
      Node::Mul(a, b) => a.eval() * b.eval(),
      Node::Lit(i) => *i,
    }
  }

  fn add<A, B>(a: A, b: B) -> Node
  where
    A: IntoNode,
    B: IntoNode,
  {
    Node::Add(Box::new(a.into()), Box::new(b.into()))
  }
  fn mul<A, B>(a: A, b: B) -> Node
  where
    A: IntoNode,
    B: IntoNode,
  {
    Node::Mul(Box::new(a.into()), Box::new(b.into()))
  }

  pub fn parse(input: &str) -> Node {
    let stripped = input.split_whitespace().collect::<String>();
    let input = stripped.as_str();
    let mut paren_count = 0;
    if let Ok(i) = input.parse() {
      return Node::Lit(i);
    }
    if input.len() == 0 {
      panic!("empty string")
    }
    let mut idx = input.len();
    for c in input.chars().rev() {
      idx -= 1;
      match c {
        ')' => {
          paren_count += 1;
        }
        '(' => {
          paren_count -= 1;
          if paren_count < 0 {
            panic!("mismatched parens")
          }
          if paren_count == 0 {
            if idx == 0 {
              return Node::parse(&input[1..(input.len() - 1)]);
            }
          }
        }
        '+' => {
          if paren_count == 0 {
            return Node::Add(
              Box::new(Node::parse(&input[..idx])),
              Box::new(Node::parse(&input[(idx + 1)..])),
            );
          } else {
            continue;
          }
        }
        '*' => {
          if paren_count == 0 {
            return Node::Mul(
              Box::new(Node::parse(&input[..idx])),
              Box::new(Node::parse(&input[(idx + 1)..])),
            );
          } else {
            continue;
          }
        }
        _ => continue,
      }
    }
    panic!("reached end of string")
  }
}

impl IntoNode for Node {
  fn into(self) -> Node {
    self
  }
}

impl IntoNode for usize {
  fn into(self) -> Node {
    Node::Lit(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(Node::parse("1"), Node::Lit(1));
    assert_eq!(Node::parse("(1)"), Node::Lit(1));
    assert_eq!(Node::parse("1+2"), Node::add(1, 2));
    assert_eq!(Node::parse("(1+2)"), Node::add(1, 2));
    assert_eq!(Node::parse("1*2"), Node::mul(1, 2));
    assert_eq!(
      Node::parse("1+2+3"),
      Node::add(Node::add(1, 2), Node::Lit(3))
    );
    assert_eq!(
      Node::parse("1+(2+3)"),
      Node::add(Node::Lit(1), Node::add(2, 3))
    );
    assert_eq!(
      Node::parse("(1+2)+3"),
      Node::add(Node::add(1, 2), Node::Lit(3))
    );
    assert_eq!(Node::parse("1 + 2 * 3 + 4 * 5 + 6").eval(), 71);
    assert_eq!(Node::parse("2 * 3 + (4 * 5)").eval(), 26);
    assert_eq!(Node::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").eval(), 437);
    assert_eq!(
      Node::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").eval(),
      12240
    );
    assert_eq!(
      Node::parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").eval(),
      13632
    );
  }
}

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Seat {
  Floor,
  Empty,
  Full,
}

impl std::fmt::Display for Seat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      Seat::Empty => write!(f, "L"),
      Seat::Full => write!(f, "#"),
      Seat::Floor => write!(f, "."),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boat {
  width: usize,
  height: usize,
  seats: Vec<Seat>,
}

impl Boat {
  pub fn parse(input: &str) -> Boat {
    let lines: Vec<&str> = input.lines().collect();
    Boat {
      width: lines[0].len(),
      height: lines.len(),
      seats: input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
          'L' => Seat::Empty,
          '#' => Seat::Full,
          '.' => Seat::Floor,
          s => panic!("bad seat: {}", s),
        })
        .collect(),
    }
  }

  fn get_index(&self, row: usize, column: usize) -> usize {
    (row * self.width + column) as usize
  }

  fn neighbor_count_index(&self, idx: usize) -> usize {
    self.neighbor_count(idx % self.width, idx / self.width)
  }

  fn neighbor_count(&self, x: usize, y: usize) -> usize {
    let mut i = 0;
    let xmin = if x == 0 { x } else { x - 1 };
    let ymin = if y == 0 { x } else { y - 1 };
    // println!("{:?}", self);
    let xmax = if x == self.width - 1 { x } else { x + 1 };
    let ymax = if y == self.height - 1 { y } else { y + 1 };
    // println!("{}, {}, {}, {}, {}, {}", x, y, xmin, xmax, ymin, ymax);
    for col in xmin..=xmax {
      for row in ymin..=ymax {
        if x == col && y == row {
          continue;
        }
        let index = self.get_index(row, col);
        // println!("{}, {}, {}, {}", row, col, index, self.seats[index]);
        if let Some(Seat::Full) = self.seats.get(index) {
          i += 1;
        }
      }
    }
    i
  }

  pub fn capacity(self) -> usize {
    // println!("{}", self);
    // println!("\n---\n");
    let mutated = self.mutated();
    if self.eq(&mutated) {
      let floor = self
        .seats
        .iter()
        .filter(|s| match *s {
          Seat::Floor => true,
          _ => false,
        })
        .count();
      let empty = self
        .seats
        .iter()
        .filter(|s| match *s {
          Seat::Empty => true,
          _ => false,
        })
        .count();
      let full = self
        .seats
        .iter()
        .filter(|s| match *s {
          Seat::Full => true,
          _ => false,
        })
        .count();
      println!("{}, {}, {}", floor, empty, full);
      return full;
    }
    mutated.capacity()
  }

  fn mutated(&self) -> Boat {
    Boat {
      width: self.width,
      height: self.height,
      seats: self
        .seats
        .iter()
        .enumerate()
        .map(|(idx, s)| match (s, self.neighbor_count_index(idx)) {
          (Seat::Floor, _) => Seat::Floor,
          (Seat::Empty, 0) => Seat::Full,
          (Seat::Full, x) if x >= 4 => Seat::Empty,
          (otherwise, _) => otherwise.clone(),
        })
        .collect(),
    }
  }
}

impl Display for Boat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    for c in self.seats.chunks(self.width) {
      for s in c.iter() {
        s.fmt(f)?;
      }
      f.write_str("\n")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_neighbor_count() {
    // L.L
    // ###
    // .#L
    let boat = Boat::parse("L.L\n###\n.#L");
    assert_eq!(boat.neighbor_count(0, 0), 2);
    assert_eq!(boat.neighbor_count(0, 1), 2);
    assert_eq!(boat.neighbor_count(1, 1), 3);
    assert_eq!(boat.neighbor_count(2, 1), 2);

    // .##
    // .##
    // #..
    let boat = Boat::parse(".##\n.##\n#..");
    assert_eq!(boat.neighbor_count(1, 1), 4);
  }

  #[test]
  fn test_cycles() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let res = Boat::parse(input).capacity();
    assert_eq!(res, 37)
  }
}

use regex::Regex;
#[macro_use]
extern crate lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
  North(usize),
  South(usize),
  East(usize),
  West(usize),
  Left(usize),
  Right(usize),
  Forward(usize),
}

impl Action {
  pub fn parse(line: &str) -> Option<Action> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^([A-Z])(\d+)$").unwrap();
    }
    let caps = RE.captures(line)?;
    let op = caps.get(1)?.as_str();
    let val = caps.get(2)?.as_str().parse().ok()?;
    match op {
      "N" => Some(Action::North(val)),
      "S" => Some(Action::South(val)),
      "E" => Some(Action::East(val)),
      "W" => Some(Action::West(val)),
      "L" => Some(Action::Left(val)),
      "R" => Some(Action::Right(val)),
      "F" => Some(Action::Forward(val)),
      _ => None,
    }
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Boat {
  x: f32,
  y: f32,
  waypoint_x: f32,
  waypoint_y: f32,
}

impl Boat {
  pub fn manhattan(&self) -> f32 {
    self.x.abs() + self.y.abs()
  }

  pub fn new() -> Boat {
    Boat {
      x: 0.0,
      y: 0.0,
      waypoint_x: 10.0,
      waypoint_y: 1.0,
    }
  }

  fn rotate(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let mag = (x.powf(2.0) + y.powf(2.0)).sqrt();
    let theta = y.atan2(x).to_degrees();
    // let theta = (y / x).atan().to_degrees();
    let new_theta = (theta - angle) % 360.0;
    // println!("mag {}, theta {}, new_theta {}", mag, theta, new_theta);
    (
      mag * new_theta.to_radians().cos(),
      mag * new_theta.to_radians().sin(),
    )
  }

  pub fn apply(boat: &Boat, action: Action) -> Boat {
    let mut x = boat.x;
    let mut y = boat.y;
    let mut waypoint_x = boat.waypoint_x;
    let mut waypoint_y = boat.waypoint_y;
    match action {
      Action::North(i) => waypoint_y += i as f32,
      Action::South(i) => waypoint_y -= i as f32,
      Action::West(i) => waypoint_x -= i as f32,
      Action::East(i) => waypoint_x += i as f32,
      Action::Left(i) => {
        let (new_x, new_y) = Boat::rotate(waypoint_x, waypoint_y, i as f32 * -1.0);
        waypoint_x = new_x;
        waypoint_y = new_y;
      }
      Action::Right(i) => {
        let (new_x, new_y) = Boat::rotate(waypoint_x, waypoint_y, i as f32);
        waypoint_x = new_x;
        waypoint_y = new_y;
      }
      Action::Forward(i) => {
        x += waypoint_x * i as f32;
        y += waypoint_y * i as f32;
      }
    }
    Boat {
      x,
      y,
      waypoint_x,
      waypoint_y,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let input = "F10
N3
F7
L90
L180
F3
R180
F3
R90
R90
F11"
      .lines()
      .map(|l| Action::parse(l).unwrap());
    let mut boat = Boat::new();
    for a in input {
      println!("{:?}", a);
      boat = Boat::apply(&boat, a);
      println!("{:?}", boat);
    }
    assert_eq!(286.0, boat.manhattan())
  }

  fn veq(a: (f32, f32), b: (f32, f32)) {
    assert!((a.0 - b.0).abs() < 0.1, format!("{:?} != {:?}", a, b));
    assert!((a.1 - b.1).abs() < 0.1, format!("{:?} != {:?}", a, b));
  }
  #[test]
  fn test_rotate() {
    veq(Boat::rotate(0.0, 10.0, 90.0), (10.0, 0.0));
    veq(Boat::rotate(0.0, 10.0, 180.0), (0.0, -10.0));
    // rotate left
    veq(Boat::rotate(4.0, 10.0, 90.0), (10.0, -4.0));
    // rotate right
    veq(Boat::rotate(4.0, 10.0, -90.0), (-10.0, 4.0));
    veq(Boat::rotate(1.0, 1.0, -180.0), (-1.0, -1.0));

    veq(Boat::rotate(-1.0, 1.0, -180.0), (1.0, -1.0));
  }
}

pub fn crabby(input: &Vec<usize>, total_size: usize, loops: usize) -> Vec<usize> {
  let orig = input.clone();
  let len = input.len();
  let mut v = input.clone();
  v.extend((len + 1)..=total_size);
  for i in 0..loops {
    println!("{}", i);
    v = shuffle(v);
  }
  let p = v.iter().position(|&e| e == 1).unwrap();
  v.rotate_left((p + 1) % total_size);
  v
}

fn shuffle(cups: Vec<usize>) -> Vec<usize> {
  let hand = &cups[1..4];
  let at = cups[0];
  let mut destination = cups[0];
  loop {
    destination -= 1;
    if destination < 1 {
      destination = cups.len();
    }
    if !hand.contains(&destination) {
      break;
    }
  }
  // println!(
  //   "cups: {:?}, pick up: {:?}\n destination: {}",
  //   cups, hand, destination
  // );
  println!(
    "at: {}, pick up: {:?}\n destination: {}",
    at, hand, destination
  );
  let p = cups.iter().skip(4).position(|&e| e == destination).unwrap();
  let p = p + 4;
  let mut shuffled = cups[4..=p].to_vec();
  shuffled.extend_from_slice(hand);
  shuffled.extend_from_slice(&cups[p + 1..]);
  shuffled.extend_from_slice(&cups[0..1]);
  let mut q = shuffled.clone();
  let p = q.iter().position(|&e| e == 1).unwrap();
  q.rotate_left((p) % cups.len());
  println!("got {:?}", q);
  shuffled
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    println!("len: {}", input.len());
    let res = crabby(&input, 200, 400);
    // let res = crabby(input, 1_000_000, 10_000_000);
    println!("res: {:?}", res);
  }
}

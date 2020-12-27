use regex::Regex;

use std::convert::AsRef;
use strum_macros::AsRefStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::collections::HashMap;

#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab_case")]
enum Field {
  BYR,
  IYR,
  EYR,
  HGT,
  HCL,
  ECL,
  PID,
}

impl Field {
  fn valid(&self, val: &str) -> bool {
    match self {
      Field::BYR => (1920..=2002).contains(&val.parse().unwrap_or(0)),
      Field::IYR => (2010..=2020).contains(&val.parse().unwrap_or(0)),
      Field::EYR => (2020..=2030).contains(&val.parse().unwrap_or(0)),
      Field::HGT => {
        let re = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        match re.captures(val) {
          None => false,
          Some(s) => {
            let unit = s.get(2).unwrap().as_str();
            let amt: i32 = s.get(1).unwrap().as_str().parse().unwrap();
            (unit == "in" && (59..=76).contains(&amt)) || (unit == "cm" && (150..=193).contains(&amt))
          }
        }
      },
      Field::HCL => Regex::new(r"^#[a-f0-9]{6}$").unwrap().is_match(val),
      Field::ECL => Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap().is_match(val),
      Field::PID => Regex::new(r"^[0-9]{9}$").unwrap().is_match(val),
    }
  }
}

pub fn valid_passports(input: &str) -> usize {
  let passports = input.split("\n\n");
  passports.filter(|x| valid_passport(x)).count()
}

fn valid_passport(input: &str) -> bool {
  let re = Regex::new(r"(\S+):(\S+)").unwrap();
  let map: HashMap<_, _> = re.captures_iter(input).map(|c| {
    (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str())
  }).collect();
  Field::iter().all(|e| {
    match map.get(e.as_ref()) {
      None => false,
      Some(v) => e.valid(v),
    }
  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
      assert!(Field::BYR.valid("2002"));
      assert!(!Field::BYR.valid("2003"));

      assert!(Field::HGT.valid("60in"));
      assert!(Field::HGT.valid("190cm"));
      assert!(!Field::HGT.valid("190in"));
      assert!(!Field::HGT.valid("190"));

      assert!(Field::HCL.valid("#123abc"));
      assert!(!Field::HCL.valid("#123abz"));
      assert!(!Field::HCL.valid("123abc"));

      assert!(Field::ECL.valid("brn"));
      assert!(!Field::ECL.valid("wat"));

      assert!(Field::PID.valid("000000001"));
      assert!(!Field::PID.valid("0123456789"));
    }

    #[test]
    fn test_valid_passports() {
      let test = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
      assert_eq!(valid_passports(test), 2);
    }
}


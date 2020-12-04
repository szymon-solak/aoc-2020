// https://adventofcode.com/2020/day/4

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

struct Passport {
    fields: Vec<(String, String)>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        if self.fields.len() == 8 {
            return true;
        }
        if self.fields.len() == 7
            && self
                .fields
                .iter()
                .find(|(field, _)| field == "cid")
                .is_none()
        {
            return true;
        }

        false
    }

    pub fn is_strictly_valid(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let as_hash: HashMap<String, String> = self.fields.iter().cloned().collect();

        lazy_static! {
            static ref BYR_REGEX: Regex = Regex::new(r"^(19[2-9]\d|200[0-2])$").unwrap();
        }
        if BYR_REGEX.captures(as_hash.get("byr").unwrap()).is_none() {
            return false;
        }

        lazy_static! {
            static ref IYR_REGEX: Regex = Regex::new(r"^(201\d|2020)$").unwrap();
        }
        if IYR_REGEX.captures(as_hash.get("iyr").unwrap()).is_none() {
            return false;
        }

        lazy_static! {
            static ref EYR_REGEX: Regex = Regex::new(r"^(202\d|2030)$").unwrap();
        }
        if EYR_REGEX.captures(as_hash.get("eyr").unwrap()).is_none() {
            return false;
        }

        lazy_static! {
            static ref HGT_REGEX: Regex =
                Regex::new(r"^((1[5-8]\d|19[0-3])cm)|(([5-6]\d|7[0-6])in)$").unwrap();
        }
        if HGT_REGEX.captures(as_hash.get("hgt").unwrap()).is_none() {
            return false;
        }

        lazy_static! {
            static ref HCL_REGEX: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        }
        if HCL_REGEX.captures(as_hash.get("hcl").unwrap()).is_none() {
            return false;
        }

        lazy_static! {
            static ref ECL_REGEX: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }
        if ECL_REGEX.captures(as_hash.get("ecl").unwrap()).is_none() {
            return false;
        }

        lazy_static! {
            static ref PID_REGEX: Regex = Regex::new(r"^([0-9]{9})$").unwrap();
        }
        if PID_REGEX.captures(as_hash.get("pid").unwrap()).is_none() {
            return false;
        }

        true
    }
}

impl FromStr for Passport {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split_whitespace()
            .map(|slice| {
                let mut parts = slice.split(':');

                (
                    parts.next().unwrap().to_owned(),
                    parts.next().unwrap().to_owned(),
                )
            })
            .collect::<Vec<(String, String)>>();

        Ok(Passport { fields })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path).unwrap();

    let passports = input_data
        .split("\n\n")
        .filter_map(|p| p.parse::<Passport>().ok())
        .collect::<Vec<Passport>>();

    let valid = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();

    let strictly_valid = passports
        .iter()
        .filter(|passport| passport.is_strictly_valid())
        .count();

    println!("[part 1] {:?}", valid);
    println!("[part 2] {:?}", strictly_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_passport_1() {
        // given
        let passport = "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
            .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(!is_valid);
    }

    #[test]
    fn invalid_passport_2() {
        // given
        let passport = "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"
            .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(!is_valid);
    }

    #[test]
    fn invalid_passport_3() {
        // given
        let passport =
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
                .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(!is_valid);
    }

    #[test]
    fn invalid_passport_4() {
        // given
        let passport = "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
            .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(!is_valid);
    }

    #[test]
    fn valid_passport_1() {
        // given
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"
            .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(is_valid);
    }

    #[test]
    fn valid_passport_2() {
        // given
        let passport =
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
                .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(is_valid);
    }

    #[test]
    fn valid_passport_3() {
        // given
        let passport =
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"
                .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(is_valid);
    }

    #[test]
    fn valid_passport_4() {
        // given
        let passport = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            .parse::<Passport>();

        // when
        let is_valid = passport.unwrap().is_strictly_valid();

        // then
        assert!(is_valid);
    }
}

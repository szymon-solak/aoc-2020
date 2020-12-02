// https://adventofcode.com/2020/day/2
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct PasswordPolicy {
    positions: [usize; 2],
    character: char,
}

#[derive(Debug, Clone)]
struct Password {
    policy: PasswordPolicy,
    password: String,
}

#[derive(Debug, Clone)]
struct PasswordParseError {}
impl From<std::char::ParseCharError> for PasswordParseError {
    fn from(_: std::char::ParseCharError) -> Self {
        PasswordParseError {}
    }
}
impl From<std::num::ParseIntError> for PasswordParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        PasswordParseError {}
    }
}

impl FromStr for Password {
    type Err = PasswordParseError;
    fn from_str(password_str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PASSWORD_REGEX: Regex = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)").unwrap();
        }

        PASSWORD_REGEX
            .captures(password_str)
            .ok_or(PasswordParseError {})
            .and_then(|captures| {
                Ok(Password {
                    policy: PasswordPolicy {
                        positions: [captures[1].parse()?, captures[2].parse()?],
                        character: captures[3].parse()?,
                    },
                    password: captures[4].parse().unwrap(),
                })
            })
    }
}

fn is_valid_sled_rental_password(pass: &Password) -> bool {
    let char = pass
        .password
        .chars()
        .filter(|c| c == &pass.policy.character)
        .count();

    char >= pass.policy.positions[0] && char <= pass.policy.positions[1]
}

fn is_valid_toboggan_password(pass: &Password) -> bool {
    let left = pass
        .password
        .chars()
        .nth(pass.policy.positions[0] - 1)
        .unwrap()
        == pass.policy.character;
    let right = pass
        .password
        .chars()
        .nth(pass.policy.positions[1] - 1)
        .unwrap()
        == pass.policy.character;

    left ^ right
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path).unwrap();

    let password_list = input_data
        .split('\n')
        .filter_map(|f| Result::ok(f.parse::<Password>()))
        .collect::<Vec<Password>>();

    let valid_passwords = password_list
        .iter()
        .filter(|p| is_valid_sled_rental_password(p))
        .count();

    println!("[part 1] Valid passwords: {:?}", valid_passwords);

    let valid_offical_passwords = password_list
        .iter()
        .filter(|p| is_valid_toboggan_password(p))
        .count();

    println!("[part 2] Valid passwords: {:?}", valid_offical_passwords);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_validation_when_min_entries() {
        // given
        let password = "1-3 a: abcde".parse::<Password>().unwrap();

        // when
        let is_valid = is_valid_sled_rental_password(&password);

        // then
        assert!(is_valid);
    }

    #[test]
    fn fails_validation_when_no_entires() {
        // given
        let password = "1-3 b: cdefg".parse::<Password>().unwrap();

        // when
        let is_valid = is_valid_sled_rental_password(&password);

        // then
        assert!(!is_valid);
    }

    #[test]
    fn passes_validation_when_max_entries() {
        // given
        let password = "2-9 c: ccccccccc".parse::<Password>().unwrap();

        // when
        let is_valid = is_valid_sled_rental_password(&password);

        // then
        assert!(is_valid);
    }

    #[test]
    fn passes_official_validation_when_single_letter() {
        // given
        let password = "1-3 a: abcde".parse::<Password>().unwrap();

        // when
        let is_valid = is_valid_toboggan_password(&password);

        // then
        assert!(is_valid);
    }

    #[test]
    fn fails_official_validation_when_no_letters() {
        // given
        let password = "1-3 b: cdefg".parse::<Password>().unwrap();

        // when
        let is_valid = is_valid_toboggan_password(&password);

        // then
        assert!(!is_valid);
    }

    #[test]
    fn fails_official_validation_when_letters_are_duplicated() {
        // given
        let password = "2-9 c: ccccccccc".parse::<Password>().unwrap();

        // when
        let is_valid = is_valid_toboggan_password(&password);

        // then
        assert!(!is_valid);
    }
}

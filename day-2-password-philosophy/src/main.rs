// https://adventofcode.com/2020/day/2

fn split(pass: &str) -> Option<(usize, usize, char, &str)> {
    let parts = pass.split(':').map(|p| p.trim()).collect::<Vec<&str>>();

    let rule = parts.get(0);
    let password = parts.get(1);

    if rule.is_none() || password.is_none() {
        return None;
    }

    let normalized = rule.unwrap().replace("-", " ");
    let rule_parts = normalized.split_whitespace().collect::<Vec<&str>>();

    let min: usize = rule_parts.get(0).unwrap().parse().unwrap();
    let max: usize = rule_parts.get(1).unwrap().parse().unwrap();
    let letter: char = rule_parts.get(2).unwrap().parse().unwrap();

    Some((min, max, letter, password.unwrap()))
}
trait Password {
    fn parse(pass: &str) -> Option<&Self>;
}

struct SledRentalPassword {}
impl Password for SledRentalPassword {
    fn parse(pass: &str) -> Option<&Self> {
        let (min, max, letter, password) = {
            let parts = split(pass);
            if parts.is_none() { return None }
            parts.unwrap()
        };

        match password.chars().filter(|f| { f == &letter }).count() {
            count if count >= min && count <= max => Some(&Self {}),
            _ => None,
        }
    }
}

struct OfficialTobogganCorporateAutheticationSystemPassword {}
impl Password for OfficialTobogganCorporateAutheticationSystemPassword {
    fn parse(pass: &str) -> Option<&Self> {
        let (left, right, letter, password) = {
            let parts = split(pass);
            if parts.is_none() { return None }
            parts.unwrap()
        };

        let left = password.chars().nth(left - 1).unwrap();
        let right = password.chars().nth(right - 1).unwrap();

        match (left, right) {
            (l ,r) if l == letter && r == letter => None,
            (l, r) if l != letter && r != letter => None,
            _ => Some(&Self {}),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path).unwrap();

    let password_list = input_data.split('\n').collect::<Vec<&str>>();

    let valid_passwords = password_list
        .iter()
        .filter_map(|f| SledRentalPassword::parse(f))
        .count();

    println!("[part 1] Valid passwords: {:?}", valid_passwords);

    let valid_offical_passwords = password_list
        .iter()
        .filter_map(|f| OfficialTobogganCorporateAutheticationSystemPassword::parse(f))
        .count();

    println!("[part 2] Valid passwords: {:?}", valid_offical_passwords);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_validation_when_min_entries() {
        // given
        let password = "1-3 a: abcde";

        // when
        let parsed = SledRentalPassword::parse(password);

        // then
        assert!(parsed.is_some());
    }

    #[test]
    fn fails_validation_when_no_entires() {
        // given
        let password = "1-3 b: cdefg";

        // when
        let parsed = SledRentalPassword::parse(password);

        // then
        assert!(parsed.is_none());
    }

    #[test]
    fn passes_validation_when_max_entries() {
        // given
        let password = "2-9 c: ccccccccc";

        // when
        let parsed = SledRentalPassword::parse(password);

        // then
        assert!(parsed.is_some());
    }

    #[test]
    fn passes_official_validation_when_single_letter() {
        // given
        let password = "1-3 a: abcde";

        // when
        let parsed = OfficialTobogganCorporateAutheticationSystemPassword::parse(password);

        // then
        assert!(parsed.is_some());
    }

    #[test]
    fn fails_official_validation_when_no_letters() {
        // given
        let password = "1-3 b: cdefg";

        // when
        let parsed = OfficialTobogganCorporateAutheticationSystemPassword::parse(password);

        // then
        assert!(parsed.is_none());
    }

    #[test]
    fn fails_official_validation_when_letters_are_duplicated() {
        // given
        let password = "2-9 c: ccccccccc";

        // when
        let parsed = OfficialTobogganCorporateAutheticationSystemPassword::parse(password);

        // then
        assert!(parsed.is_none());
    }
}

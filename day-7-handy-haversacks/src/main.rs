// https://adventofcode.com/2020/day/7

use lazy_static::lazy_static;
use regex::Regex;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug)]
struct Rule {
    bag_type: String,
    can_contain: Vec<(String, i32)>,
}

impl Rule {
    pub fn can_hold(&self, bag: &str, rules: &[Rule]) -> bool {
        if self.can_contain.is_empty() {
            return false;
        }

        if self.can_contain.iter().any(|(bag_type, _)| bag_type == bag) {
            // println!("matched rule for ({:?})", self);
            return true;
        }

        self.can_contain
            .iter()
            .filter_map(|(bag_type, _)| rules.iter().find(|r| &r.bag_type == bag_type))
            .any(|rule| rule.can_hold(bag, rules))
    }

    pub fn count_nested_bags(&self, rules: &[Rule]) -> i32 {
        self.can_contain
            .iter()
            .map(|(bag_type, amount)| {
                let nested_rule = rules.iter().find(|r| &r.bag_type == bag_type).unwrap();
                amount + (amount * nested_rule.count_nested_bags(rules))
            })
            .sum()
    }
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BAG_REGEX: Regex = Regex::new(r"(^|\d)\s?(.+?)(?:\s+bags?)").unwrap();
        }

        let mut captures = BAG_REGEX.captures_iter(s);
        let bag_type = captures.next().unwrap()[2].trim().to_owned();

        let can_contain = captures
            .collect::<Vec<regex::Captures>>()
            .iter()
            .map(|cap| (cap[2].to_owned(), cap[1].parse::<i32>().unwrap()))
            .map(|(bag, amount)| (bag, amount))
            .collect::<Vec<(String, i32)>>();

        Ok(Rule {
            bag_type,
            can_contain,
        })
    }
}

struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn how_many_can_hold(&self, bag: &str) -> usize {
        self.rules
            .iter()
            // .inspect(|r| println!("Base rules: {:?}", r))
            .filter(|r| r.can_hold(bag, &self.rules))
            .count()
    }

    pub fn count_nested_bags_for(&self, bag: &str) -> i32 {
        let root_rule = self.rules.iter().find(|r| r.bag_type == bag).unwrap();

        root_rule.count_nested_bags(&self.rules)
    }
}

impl FromIterator<Rule> for RuleSet {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        RuleSet {
            rules: Vec::from_iter(iter),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path).unwrap();

    let rules = input_data
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

    println!("[part 1] {:?}", rules.how_many_can_hold("shiny gold"));
    println!("[part 2] {:?}", rules.count_nested_bags_for("shiny gold"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_count_when_the_bag_is_on_the_left_side() {
        // given
        let example_rules = "
        shiny gold bags contain 1 bright white bag, 2 dark olive bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let can_hold_shiny_gold_amount = example_rules.how_many_can_hold("shiny gold");

        // then
        assert_eq!(can_hold_shiny_gold_amount, 0)
    }

    #[test]
    fn should_count_when_the_bag_is_on_the_right_side() {
        // given
        let example_rules = "
            dark olive bags contain 1 bright white bag, 2 shiny gold bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let can_hold_shiny_gold_amount = example_rules.how_many_can_hold("shiny gold");

        // then
        assert_eq!(can_hold_shiny_gold_amount, 1)
    }

    #[test]
    fn should_count_when_the_bag_is_nested() {
        // given
        let example_rules = "
            dark olive bags contain 1 bright white bag, 2 light red bags.
            light red bags contain 2 shiny gold bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let can_hold_shiny_gold_amount = example_rules.how_many_can_hold("shiny gold");

        // then
        assert_eq!(can_hold_shiny_gold_amount, 2)
    }

    #[test]
    fn part1_example() {
        // given
        let example_rules = "
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let can_hold_shiny_gold_amount = example_rules.how_many_can_hold("shiny gold");

        // then
        assert_eq!(can_hold_shiny_gold_amount, 4);
    }

    #[test]
    fn part2_example_1() {
        // given
        let example_rules = "
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let nested_bags = example_rules.count_nested_bags_for("shiny gold");

        // then
        assert_eq!(nested_bags, 32)
    }

    #[test]
    fn can_count_bags_for_simple_case() {
        // given
        let example_rules = "
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let nested_bags = example_rules.count_nested_bags_for("dark olive");

        // then
        // (3 + 4)
        assert_eq!(nested_bags, 7)
    }

    #[test]
    fn can_count_bags_for_multiplied_branches() {
        // given
        let example_rules = "
            vibrant plum bags contain 2 dark olive bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            faded blue bags contain 5 dotted black bags.
            dotted black bags contain no other bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let nested_bags = example_rules.count_nested_bags_for("vibrant plum");

        // then
        // 2 + 2*(3 + (3 * 5) + 4)
        assert_eq!(nested_bags, 46)
    }

    #[test]
    fn part2_example_2() {
        // given
        let example_rules = "
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        "
        .trim()
        .lines()
        .map(|r| r.parse::<Rule>().unwrap())
        .collect::<RuleSet>();

        // when
        let nested_bags = example_rules.count_nested_bags_for("shiny gold");

        // then
        assert_eq!(nested_bags, 126)
    }
}

use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

fn parse_input(input: &str) -> Vec<(String, Vec<(usize, String)>)> {
    let no_bags_pattern = Regex::new(r"([a-z]+ [a-z]+) bags contain no other bags.").unwrap();
    let bags_pattern =
        Regex::new(r"([a-z]+ [a-z]+) bags contain(( [0-9]+ [a-z]+ [a-z]+ bags?[,.])+)").unwrap();
    let bag_pattern = Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bags?").unwrap();

    input
        .lines()
        .map(|line| {
            if let Some(caps) = bags_pattern.captures(line) {
                let main_bag = caps
                    .get(1)
                    .expect("should have at least one group")
                    .as_str()
                    .to_owned();
                let other_bags = caps.get(2).expect("should have bag groups").as_str();
                let sub_bags: Vec<_> = bag_pattern
                    .captures_iter(other_bags)
                    .map(|caps| {
                        let count = caps
                            .get(1)
                            .expect("cound should be matched")
                            .as_str()
                            .parse::<usize>()
                            .expect("count should be an int");
                        let bag = caps
                            .get(2)
                            .expect("bag should be matched")
                            .as_str()
                            .to_owned();
                        (count, bag)
                    })
                    .collect();
                (main_bag, sub_bags)
            } else if let Some(caps) = no_bags_pattern.captures(line) {
                (
                    caps.get(1)
                        .expect("should have at least one group")
                        .as_str()
                        .to_owned(),
                    Default::default(),
                )
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn count_outermost(rules: &[(String, Vec<(usize, String)>)], bag: &str) -> usize {
    let mut is_inside: HashMap<&str, Vec<&str>> = Default::default();
    for (outer_bag, inner_bags) in rules {
        for (_, inner_bag) in inner_bags {
            is_inside.entry(inner_bag).or_default().push(outer_bag);
        }
    }

    let mut found = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(bag);
    while let Some(elem) = queue.pop_front() {
        if found.contains(elem) {
            continue;
        }
        found.insert(elem);
        if let Some(inner_bags) = is_inside.get(elem) {
            for bag in inner_bags {
                queue.push_back(*bag);
            }
        }
    }
    found.len() - 1
}

fn count_inner(rules: &[(String, Vec<(usize, String)>)], bag: &str) -> usize {
    rules
        .iter()
        .filter(|(rule_bag, _)| bag == rule_bag)
        .map(|(_, contents)| {
            contents
                .into_iter()
                .map(|(count, bag)| count_inner(rules, bag) * count)
                .sum::<usize>()
                + 1
        })
        .sum()
}

fn main() {
    let rules = parse_input(include_str!("../../inputs/day07.txt"));
    println!("Part 1: {}", count_outermost(&rules, "shiny gold"));
    println!("Part 1: {}", count_inner(&rules, "shiny gold") - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input("dim tan bags contain no other bags."),
            vec![("dim tan".to_owned(), vec![])]
        );
        assert_eq!(parse_input("bright crimson bags contain 4 dull gold bags, 1 dim lime bag, 2 plaid crimson bags, 3 pale gold bags."), vec![
            ("bright crimson".to_owned(), vec![
                (4, "dull gold".to_owned()),
                (1, "dim lime".to_owned()),
                (2, "plaid crimson".to_owned()),
                (3, "pale gold".to_owned()),
            ])
        ]);
    }

    #[test]
    fn test_part1_sample() {
        let rules = parse_input(include_str!("../../inputs/day07-sample.txt"));
        assert_eq!(count_outermost(&rules, "shiny gold"), 4);
    }

    #[test]
    fn test_part2_sample() {
        let rules = parse_input(include_str!("../../inputs/day07-sample.txt"));
        assert_eq!(count_inner(&rules, "shiny gold"), 33);
    }
}

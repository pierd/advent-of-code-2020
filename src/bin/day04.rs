use std::collections::HashMap;

use regex::Regex;

fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    let mut result = Vec::new();
    let mut current_map = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            if !current_map.is_empty() {
                result.push(current_map);
                current_map = HashMap::new();
            }
        } else {
            for pair in line.split_whitespace() {
                let mut parts = pair.split(':').take(2);
                current_map.insert(
                    parts.next().expect("expected 1st element"),
                    parts.next().expect("expected 2nd element"),
                );
            }
        }
    }
    if !current_map.is_empty() {
        result.push(current_map);
    }

    result
}

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn create_dummy_validation() -> HashMap<&'static str, Box<dyn Fn(&str) -> bool>> {
    let mut map: HashMap<&'static str, Box<dyn Fn(&str) -> bool>> = HashMap::new();
    for field in REQUIRED_FIELDS {
        map.insert(*field, Box::new(move |_: &str| true));
    }
    map
}

fn is_valid_passport(
    passport: &HashMap<&str, &str>,
    validation: &HashMap<&str, Box<dyn Fn(&str) -> bool>>,
) -> bool {
    for (field, validation_fn) in validation.iter() {
        if passport.get(field).map(|s| validation_fn(s)) != Some(true) {
            return false;
        }
    }
    true
}

fn create_year_validator(from: usize, to: usize) -> Box<dyn Fn(&str) -> bool> {
    assert!(from <= to);
    Box::new(move |s: &str| {
        s.parse::<usize>()
            .map(|yr| from <= yr && yr <= to)
            .unwrap_or_default()
    })
}

fn create_regex_validator(re: Regex) -> Box<dyn Fn(&str) -> bool> {
    Box::new(move |s: &str| re.is_match(s))
}

fn create_validation() -> HashMap<&'static str, Box<dyn Fn(&str) -> bool>> {
    let mut map = HashMap::new();
    map.insert("byr", create_year_validator(1920, 2002));
    map.insert("iyr", create_year_validator(2010, 2020));
    map.insert("eyr", create_year_validator(2020, 2030));

    let re = Regex::new(r"^([0-9]+)(in|cm)$").unwrap();
    map.insert(
        "hgt",
        Box::new(move |s: &str| {
            if let Some(caps) = re.captures(s) {
                if let Some(h) = caps.get(1).and_then(|m| m.as_str().parse::<usize>().ok()) {
                    let unit = caps.get(2).map(|m| m.as_str());
                    match unit {
                        Some("cm") => (150..=193).contains(&h),
                        Some("in") => (59..=76).contains(&h),
                        _ => false,
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }),
    );

    map.insert(
        "hcl",
        create_regex_validator(Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
    );
    map.insert(
        "ecl",
        create_regex_validator(Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap()),
    );
    map.insert(
        "pid",
        create_regex_validator(Regex::new(r"^[0-9]{9}$").unwrap()),
    );
    map
}

fn main() {
    let passports = parse_input(include_str!("../../inputs/day04.txt"));
    let dummy_validation = create_dummy_validation();
    println!(
        "Part 1: {}",
        passports
            .iter()
            .filter(|p| is_valid_passport(*p, &dummy_validation))
            .count()
    );
    let proper_validation = create_validation();
    println!(
        "Part 2: {}",
        passports
            .iter()
            .filter(|p| is_valid_passport(*p, &proper_validation))
            .count()
    );
}

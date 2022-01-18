use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if !current.is_empty() {
                result.push(current);
                current = Vec::new();
            }
        } else {
            current.push(line.chars().collect::<HashSet<_>>());
        }
    }
    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn sum_sets(sets: &[HashSet<char>]) -> HashSet<char> {
    let mut result = HashSet::new();
    for set in sets {
        result.extend(set);
    }
    result
}

fn intersect_sets(sets: &[HashSet<char>]) -> HashSet<char> {
    if let Some((first, rest)) = sets.split_first() {
        let mut result = first.clone();
        for set in rest {
            result = result.intersection(set).cloned().collect();
        }
        result
    } else {
        Default::default()
    }
}

fn main() {
    let groups = parse_input(include_str!("../../inputs/day06.txt"));
    println!(
        "Part 1: {}",
        groups
            .iter()
            .map(|sets| sum_sets(sets).len())
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        groups
            .iter()
            .map(|sets| intersect_sets(sets).len())
            .sum::<usize>()
    );
}

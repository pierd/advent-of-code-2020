use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<HashSet<char>> {
    let mut result = Vec::new();
    let mut current = HashSet::new();

    for line in input.lines() {
        if line.is_empty() {
            if !current.is_empty() {
                result.push(current);
                current = Default::default();
            }
        } else {
            current.extend(line.chars());
        }
    }
    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn parse_input_intersected(input: &str) -> Vec<HashSet<char>> {
    let mut result = Vec::new();
    let mut current: Option<HashSet<char>> = None;

    for line in input.lines() {
        if line.is_empty() {
            if let Some(set) = current {
                result.push(set);
                current = None;
            }
        } else {
            let new = line.chars().collect::<HashSet<_>>();
            current = Some(match current.take() {
                None => new,
                Some(set) => set.intersection(&new).cloned().collect(),
            });
        }
    }
    if let Some(set) = current {
        result.push(set);
    }

    result
}

fn main() {
    let answer_groups = parse_input(include_str!("../../inputs/day06.txt"));
    println!(
        "Part 1: {}",
        answer_groups.iter().map(HashSet::len).sum::<usize>()
    );

    let correct_answer_groups = parse_input_intersected(include_str!("../../inputs/day06.txt"));
    println!(
        "Part 2: {}",
        correct_answer_groups
            .iter()
            .map(HashSet::len)
            .sum::<usize>()
    );
}

use std::collections::HashSet;
use std::mem;
use std::str::FromStr;

use std::ops::{Deref, RangeInclusive};

struct Ticket(Vec<usize>);

impl Deref for Ticket {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| ())?,
        ))
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

fn parse_range(s: &str) -> Result<RangeInclusive<usize>, ()> {
    let mut parts = s.split('-').map(|s| s.parse::<usize>().map_err(|_| ()));
    Ok((parts.next().ok_or(())??)..=(parts.next().ok_or(())??))
}

impl Rule {
    fn is_departure(&self) -> bool {
        self.name.starts_with("departure ")
    }

    fn in_range(&self, v: &usize) -> bool {
        self.ranges.iter().any(|range| range.contains(v))
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let name = parts.next().ok_or(())?.to_owned();
        let raw_ranges = parts.next().ok_or(())?.trim();
        let ranges = raw_ranges
            .split(" or ")
            .map(parse_range)
            .collect::<Result<_, _>>()?;
        Ok(Self { name, ranges })
    }
}

struct Input {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Input {
    fn is_valid_field_value(&self, val: usize) -> bool {
        self.rules.iter().any(|rule| rule.in_range(&val))
    }

    fn find_invalid_fields(&self) -> impl Iterator<Item = usize> + '_ {
        self.nearby_tickets
            .iter()
            .flat_map(|t| t.iter())
            .cloned()
            .filter(|v| !self.is_valid_field_value(*v))
    }

    fn discard_invalid_tickets(&mut self) {
        // stealing the vec for a second to modify it while still using an immutable ref to self for filtering
        let mut temp = mem::take(&mut self.nearby_tickets);
        temp.retain(|t| t.iter().all(|v| self.is_valid_field_value(*v)));
        self.nearby_tickets = temp;
    }

    fn figure_out_rules(&self) -> Vec<&Rule> {
        let mut possible_rules: Vec<HashSet<&Rule>> =
            vec![Default::default(); self.your_ticket.len()];
        for rule in &self.rules {
            for (idx, possible_rules_set) in possible_rules.iter_mut().enumerate() {
                if self
                    .nearby_tickets
                    .iter()
                    .all(|ticket| rule.in_range(&ticket[idx]))
                {
                    possible_rules_set.insert(rule);
                }
            }
        }

        let mut figured_out: Vec<Option<&Rule>> = vec![None; self.your_ticket.len()];
        while figured_out.iter().any(|rule| rule.is_none()) {
            // find a not figured out rule that has only one possible rule
            if let Some((found_idx, (figured_out_target, possible_rules_set))) = figured_out
                .iter_mut()
                .zip(possible_rules.iter())
                .enumerate()
                .find(|(_, (figured_out, possible_rules_set))| {
                    figured_out.is_none() && possible_rules_set.len() == 1
                })
            {
                // write it down in figure_out_target
                let rule = *possible_rules_set
                    .iter()
                    .next()
                    .expect("there should be one - we just found it");
                *figured_out_target = Some(rule);

                // remove the rule from other possible rules sets
                for (idx, possible_rules_set) in possible_rules.iter_mut().enumerate() {
                    if idx != found_idx {
                        possible_rules_set.remove(rule);
                    }
                }
            } else {
                panic!("can't figure it out");
            }
        }

        possible_rules
            .into_iter()
            .map(|set| set.into_iter().next().unwrap())
            .collect()
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut rules = Vec::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            rules.push(line.parse()?);
        }

        // skip "your ticket:"
        if lines.next() != Some("your ticket:") {
            return Err(());
        }
        let your_ticket = lines.next().ok_or(())?.parse::<Ticket>()?;

        // skip empty line
        if lines.next() != Some("") {
            return Err(());
        }

        // skip "nearby tickets:"
        if lines.next() != Some("nearby tickets:") {
            return Err(());
        }
        let nearby_tickets = lines
            .map(|line| line.parse::<Ticket>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            rules,
            your_ticket,
            nearby_tickets,
        })
    }
}

fn main() {
    let mut input = include_str!("../../inputs/day16.txt")
        .parse::<Input>()
        .expect("input should parse correctly");
    println!("Part 1: {}", input.find_invalid_fields().sum::<usize>());

    input.discard_invalid_tickets();
    println!(
        "Part 2: {}",
        input
            .your_ticket
            .iter()
            .zip(input.figure_out_rules().iter())
            .filter_map(|(val, rule)| if rule.is_departure() { Some(val) } else { None })
            .product::<usize>()
    );
}

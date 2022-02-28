use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cups {
    next_cup: Vec<usize>,
    current: usize,
}

impl Cups {
    fn new() -> Self {
        Self {
            next_cup: vec![0; 10],
            current: 1,
        }
    }

    fn extend_to_size(&mut self, new_size: usize) {
        let last = self
            .next_cup
            .iter()
            .enumerate()
            .find(|(_, next)| **next == self.current)
            .expect("something has to point to the current")
            .0;
        let first_new_num = self.next_cup.len();
        self.next_cup.reserve(new_size - first_new_num + 1);
        for i in (first_new_num + 1)..=(new_size + 1) {
            self.next_cup.push(i);
        }
        self.next_cup[last] = first_new_num;
        self.next_cup[new_size] = self.current;
    }

    fn advance(&mut self) {
        // pick 3 cups after current
        let picked_cups = [
            self.next_cup[self.current],
            self.next_cup[self.next_cup[self.current]],
            self.next_cup[self.next_cup[self.next_cup[self.current]]],
        ];

        // remove the picked cups
        self.next_cup[self.current] = self.next_cup[picked_cups[2]];

        // find destination cup
        let last_cup = self.next_cup.len() - 1;
        let decr = |val: usize| {
            if val == 1 {
                last_cup
            } else {
                val - 1
            }
        };
        let mut destination_cup = decr(self.current);
        while picked_cups.contains(&destination_cup) {
            destination_cup = decr(destination_cup);
        }

        // insert picked cups after the destination cup
        self.next_cup[picked_cups[2]] = self.next_cup[destination_cup];
        self.next_cup[destination_cup] = picked_cups[0];

        // select next current cup
        self.current = self.next_cup[self.current];
    }

    fn labels_after_1(&self) -> String {
        let mut result = String::new();
        let mut idx = 1;
        while self.next_cup[idx] != 1 {
            result
                .write_fmt(format_args!("{}", self.next_cup[idx]))
                .expect("formatting failed");
            idx = self.next_cup[idx];
        }
        result
    }

    fn checksum(&self) -> usize {
        self.next_cup[1] * self.next_cup[self.next_cup[1]]
    }
}

impl FromStr for Cups {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .chars()
            .map(|c| (c as u8 - b'0') as usize)
            .collect::<Vec<_>>();
        let mut cups = Cups::new();
        for pair in nums.windows(2) {
            if pair[0] >= 10 || pair[1] >= 10 {
                return Err(());
            }
            cups.next_cup[pair[0]] = pair[1];
        }
        cups.next_cup[*nums.last().unwrap()] = *nums.first().unwrap();
        cups.current = *nums.first().ok_or(())?;
        Ok(cups)
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current = self.current;
        f.write_fmt(format_args!("{}", current))?;
        let mut i = self.next_cup[current];
        while i != current {
            f.write_fmt(format_args!("->{}", i))?;
            i = self.next_cup[i];
        }
        Ok(())
    }
}

fn main() {
    let mut cups = "326519478".parse::<Cups>().expect("should parse");
    for _ in 0..100 {
        cups.advance();
    }
    println!("Part 1: {}", cups.labels_after_1());

    let mut cups = "326519478".parse::<Cups>().expect("should parse");
    cups.extend_to_size(1_000_000);
    for _ in 0..10_000_000 {
        cups.advance();
    }
    println!("Part 2: {}", cups.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "123456789".parse::<Cups>().unwrap().next_cup,
            [0, 2, 3, 4, 5, 6, 7, 8, 9, 1],
        );
    }

    #[test]
    fn test_extend() {
        let mut cups = "123456789".parse::<Cups>().unwrap();
        assert_eq!(cups.next_cup, [0, 2, 3, 4, 5, 6, 7, 8, 9, 1],);
        cups.extend_to_size(12);
        assert_eq!(cups.next_cup, [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1],);
        cups.extend_to_size(15);
        assert_eq!(
            cups.next_cup,
            [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1],
        );
    }

    #[test]
    fn test_advance() {
        let mut cups = "389125467".parse::<Cups>().unwrap();
        assert_eq!(
            cups,
            Cups {
                next_cup: vec![0, 2, 5, 8, 6, 4, 7, 3, 9, 1],
                current: 3,
            }
        );
        cups.advance();
        assert_eq!(cups.current, 2);
        assert_eq!(cups.labels_after_1(), "54673289");
    }

    #[test]
    fn test_advance_after_extend() {
        let mut cups = "389125467".parse::<Cups>().unwrap();
        cups.extend_to_size(15);
        assert_eq!(
            cups.to_string(),
            "3->8->9->1->2->5->4->6->7->10->11->12->13->14->15"
        );
        cups.advance();
        assert_eq!(
            cups.to_string(),
            "2->8->9->1->5->4->6->7->10->11->12->13->14->15->3"
        );
    }

    #[test]
    fn test_labels_after_1() {
        assert_eq!(
            "123456789".parse::<Cups>().unwrap().labels_after_1(),
            "23456789"
        );
        assert_eq!(
            "326519478".parse::<Cups>().unwrap().labels_after_1(),
            "94783265"
        );
    }

    #[test]
    fn test_part1_sample() {
        let sample_cups = "389125467".parse::<Cups>().expect("should parse");

        {
            let mut cups = sample_cups.clone();
            for _ in 0..10 {
                cups.advance();
            }
            assert_eq!(cups.labels_after_1(), "92658374");
        }

        {
            let mut cups = sample_cups.clone();
            for _ in 0..100 {
                cups.advance();
            }
            assert_eq!(cups.labels_after_1(), "67384529");
        }
    }

    #[test]
    #[ignore]
    fn test_part2_sample() {
        let mut cups = "389125467".parse::<Cups>().expect("should parse");
        cups.extend_to_size(1_000_000);
        for _ in 0..10_000_000 {
            cups.advance();
        }
        assert_eq!(cups.next_cup[1], 934001);
        assert_eq!(cups.next_cup[cups.next_cup[1]], 159792);
    }
}

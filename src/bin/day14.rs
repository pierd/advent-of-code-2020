use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy, Debug, Default)]
struct CompositeMask {
    mask: usize,
    val: usize,
}

impl CompositeMask {
    fn apply(&self, v: usize) -> usize {
        v & !(self.mask & !self.val) | self.val
    }

    fn iter_apply_v2(&self, v: usize) -> CompositeMaskApplyV2Iter {
        CompositeMaskApplyV2Iter::new(self, v)
    }
}

struct CompositeMaskApplyV2Iter {
    val: usize,
    next_bits: usize,
    bits: Vec<usize>,
}

impl CompositeMaskApplyV2Iter {
    fn new(mask: &CompositeMask, base: usize) -> Self {
        let bits = {
            let mut bit_indices = Vec::new();
            let mut mask = mask.mask;
            for idx in 0..36 {
                if mask & 1 == 0 {
                    bit_indices.push(idx);
                }
                mask >>= 1;
            }
            bit_indices
        };
        Self {
            val: base & mask.mask | mask.val,
            next_bits: 0,
            bits,
        }
    }
}

impl Iterator for CompositeMaskApplyV2Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_bits == 1 << self.bits.len() {
            None
        } else {
            let mut masked = self.val;
            let mut bits = self.next_bits;
            for bit_idx in &self.bits {
                masked |= (bits & 1) << bit_idx;
                bits >>= 1;
            }
            self.next_bits += 1;
            Some(masked)
        }
    }
}

impl FromStr for CompositeMask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|c| c != 'X' && c != '0' && c != '1') {
            return Err(());
        }
        let mask = s
            .chars()
            .map(|c| if c == 'X' { 0 } else { 1 })
            .fold(0, |acc, bit| (acc << 1) | bit);
        let val = s
            .chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .fold(0, |acc, bit| (acc << 1) | bit);
        Ok(Self { mask, val })
    }
}

#[derive(Clone, Copy, Debug)]
enum Instr {
    Mask(CompositeMask),
    Set { key: usize, val: usize },
}

static MASK_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"mask = ([X01]{36})").unwrap());
static MEM_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").unwrap());

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = MASK_PATTERN.captures(s) {
            let raw_mask = caps.get(1).ok_or(())?.as_str();
            Ok(Self::Mask(raw_mask.parse()?))
        } else if let Some(caps) = MEM_PATTERN.captures(s) {
            let key = caps.get(1).ok_or(())?.as_str().parse().map_err(|_| ())?;
            let val = caps.get(2).ok_or(())?.as_str().parse().map_err(|_| ())?;
            Ok(Self::Set { key, val })
        } else {
            Err(())
        }
    }
}

fn execute_all(instrs: &[Instr]) -> HashMap<usize, usize> {
    let mut mem = HashMap::new();
    let mut mask = CompositeMask::default();
    for instr in instrs {
        match instr {
            Instr::Mask(new_mask) => {
                mask = *new_mask;
            }
            Instr::Set { key, val } => {
                mem.insert(*key, mask.apply(*val));
            }
        }
    }
    mem
}

fn execute_all_v2(instrs: &[Instr]) -> HashMap<usize, usize> {
    let mut mem = HashMap::new();
    let mut mask = CompositeMask::default();
    for instr in instrs {
        match instr {
            Instr::Mask(new_mask) => {
                mask = *new_mask;
            }
            Instr::Set { key, val } => {
                for masked_key in mask.iter_apply_v2(*key) {
                    mem.insert(masked_key, *val);
                }
            }
        }
    }
    mem
}

fn parse_input(input: &str) -> Result<Vec<Instr>, ()> {
    input
        .lines()
        .map(|line| line.parse::<Instr>())
        .collect::<Result<Vec<_>, _>>()
}

fn main() {
    let instrs: Vec<_> =
        parse_input(include_str!("../../inputs/day14.txt")).expect("input should parse");
    println!(
        "Part 1: {}",
        execute_all(&instrs).iter().map(|(_, v)| v).sum::<usize>()
    );
    println!(
        "Part 2: {}",
        execute_all_v2(&instrs)
            .iter()
            .map(|(_, v)| v)
            .sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            .parse::<CompositeMask>()
            .unwrap();
        assert_eq!(mask.mask, 66);
        assert_eq!(mask.val, 64);
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }

    #[test]
    fn test_iter_mask() {
        let mask = "000000000000000000000000000000X1001X"
            .parse::<CompositeMask>()
            .unwrap();
        let mut itr = mask.iter_apply_v2(42);
        assert_eq!(itr.next(), Some(26));
        assert_eq!(itr.next(), Some(27));
        assert_eq!(itr.next(), Some(58));
        assert_eq!(itr.next(), Some(59));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn test_iter_mask2() {
        let mask = "00000000000000000000000000000000X0XX"
            .parse::<CompositeMask>()
            .unwrap();
        let mut itr = mask.iter_apply_v2(26);
        assert_eq!(itr.next(), Some(16));
        assert_eq!(itr.next(), Some(17));
        assert_eq!(itr.next(), Some(18));
        assert_eq!(itr.next(), Some(19));
        assert_eq!(itr.next(), Some(24));
        assert_eq!(itr.next(), Some(25));
        assert_eq!(itr.next(), Some(26));
        assert_eq!(itr.next(), Some(27));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn test_part1_sample() {
        let instrs = parse_input(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0",
        )
        .unwrap();
        assert_eq!(
            execute_all(&instrs).iter().map(|(_, v)| v).sum::<usize>(),
            165
        );
    }

    #[test]
    fn test_part2_sample() {
        let instrs = parse_input("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1").unwrap();
        assert_eq!(
            execute_all_v2(&instrs)
                .iter()
                .map(|(_, v)| v)
                .sum::<usize>(),
            208
        );
    }
}

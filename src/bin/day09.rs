use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
struct StreamValidator {
    len: usize,
    num_buf: VecDeque<usize>,
    sums_count: HashMap<usize, usize>,
}

impl StreamValidator {
    fn new(len: usize) -> Self {
        assert!(len > 0);
        Self {
            len,
            num_buf: Default::default(),
            sums_count: Default::default(),
        }
    }

    fn feed_num(&mut self, num: usize) -> bool {
        // accept if there's not enough numbers or if there is a sum
        let result = self.num_buf.len() < self.len
            || self
                .sums_count
                .get(&num)
                .map(|count| *count > 0)
                .unwrap_or_default();

        if self.num_buf.len() == self.len {
            // time to drop a num from front
            let dropped = self
                .num_buf
                .pop_front()
                .expect("we already checked that there are some numbers here");
            for other_num in self.num_buf.iter() {
                if let Some(count) = self.sums_count.get_mut(&(dropped + other_num)) {
                    *count -= 1;
                }
            }
        }

        // time to add the new number
        assert!(self.num_buf.len() < self.len);
        for other_num in self.num_buf.iter() {
            *self.sums_count.entry(num + *other_num).or_default() += 1;
        }
        self.num_buf.push_back(num);

        result
    }
}

fn find_invalid(nums: &[usize]) -> Option<usize> {
    let mut validator = StreamValidator::new(25);
    nums.iter().cloned().find(|num| !validator.feed_num(*num))
}

fn find_contiguous_sum(nums: &[usize], target: usize) -> VecDeque<usize> {
    let mut buf: VecDeque<usize> = Default::default();
    let mut sum = 0;
    for n in nums {
        buf.push_back(*n);
        sum += n;
        if sum == target {
            break;
        }
        while sum > target {
            if let Some(first) = buf.pop_front() {
                sum -= first;
            }
            if sum == target {
                return buf;
            }
        }
    }
    buf
}

fn main() {
    let nums: Vec<usize> = include_str!("../../inputs/day09.txt")
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .expect("nums should parse");
    let invalid_num = find_invalid(&nums).expect("there should be a not accepted number");
    println!("Part 1: {invalid_num}");
    let contiguous_sum = find_contiguous_sum(&nums, invalid_num);
    assert_eq!(contiguous_sum.iter().sum::<usize>(), invalid_num);
    println!(
        "Part 2: {}",
        contiguous_sum.iter().min().unwrap() + contiguous_sum.iter().max().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_sample() {
        let nums = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(find_contiguous_sum(&nums, 127), vec![15, 25, 47, 40]);
    }
}

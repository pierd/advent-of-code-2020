use std::collections::VecDeque;

fn solve_joltage(nums: &[usize]) -> [usize; 4] {
    let mut result = [0usize; 4];
    let mut nums = nums.to_vec();
    nums.sort_unstable();
    let mut last = 0;
    for num in nums {
        result[num - last] += 1;
        last = num;
    }
    result
}

fn count_ways(nums: &[usize]) -> usize {
    let mut nums = nums.to_vec();
    nums.sort_unstable();
    let mut ways = VecDeque::with_capacity(3);
    ways.push_back((0, 1));
    for n in nums {
        while ways
            .front()
            .map(|(front, _)| *front + 3 < n)
            .unwrap_or_default()
        {
            ways.pop_front();
        }
        ways.push_back((n, ways.iter().map(|(_, ways_count)| *ways_count).sum()));
    }
    ways.pop_back().map(|(_, ways)| ways).unwrap_or_default()
}

fn main() {
    let nums: Vec<usize> = include_str!("../../inputs/day10.txt")
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .expect("nums should parse");
    let diffs = solve_joltage(&nums);
    println!("Part 1: {}", (diffs[3] + 1) * diffs[1]);
    println!("Part 2: {}", count_ways(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_samples() {
        assert_eq!(
            solve_joltage(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]),
            [0, 7, 0, 4]
        );
    }

    #[test]
    fn test_part2_samples() {
        assert_eq!(count_ways(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), 8);
        assert_eq!(
            count_ways(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ]),
            19208
        );
    }
}

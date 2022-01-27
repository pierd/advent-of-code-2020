use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Game {
    num_to_last_turn: HashMap<usize, usize>,
    last_num: usize,
    last_turn: usize,
}

impl Game {
    fn new(starting_nums: &[usize]) -> Self {
        let mut num_to_last_turn = HashMap::with_capacity(starting_nums.len());
        let (last_num, prev_nums) = starting_nums.split_last().unwrap();
        for (turn, num) in prev_nums.iter().enumerate() {
            num_to_last_turn.insert(*num, turn);
        }
        Self {
            num_to_last_turn,
            last_num: *last_num,
            last_turn: prev_nums.len(),
        }
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let num = if let Some(turn) = self.num_to_last_turn.get(&self.last_num) {
            self.last_turn - turn
        } else {
            0
        };
        self.num_to_last_turn.insert(self.last_num, self.last_turn);
        self.last_num = num;
        self.last_turn += 1;
        Some(num)
    }
}

fn main() {
    println!(
        "Part 1: {}",
        Game::new(&[6, 4, 12, 1, 20, 0, 16]).nth(2020 - 8).unwrap()
    );
    println!(
        "Part 1: {}",
        Game::new(&[6, 4, 12, 1, 20, 0, 16])
            .nth(30000000 - 8)
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iteration() {
        let mut game = Game::new(&[0, 3, 6]);
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(1));
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(4));
        assert_eq!(game.next(), Some(0));
    }

    #[test]
    fn test_part1_samples() {
        assert_eq!(Game::new(&[1, 3, 2]).nth(2020 - 4), Some(1));
        assert_eq!(Game::new(&[2, 1, 3]).nth(2020 - 4), Some(10));
    }

    #[test]
    #[ignore]
    fn test_part2_samples() {
        assert_eq!(Game::new(&[0, 3, 6]).nth(30000000 - 4), Some(175594));
        assert_eq!(Game::new(&[2, 1, 3]).nth(30000000 - 4), Some(3544142));
    }
}

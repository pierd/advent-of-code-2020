use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<usize>), ()> {
    let mut players = input.split("\n\n").map(|player| {
        player
            .lines()
            .skip(1)
            .map(|s| s.parse::<usize>().map_err(|_| ()))
            .collect::<Result<Vec<usize>, ()>>()
    });
    Ok((players.next().ok_or(())??, players.next().ok_or(())??))
}

fn play(player1: &[usize], player2: &[usize]) -> VecDeque<usize> {
    let mut player1: VecDeque<usize> = player1.iter().cloned().collect();
    let mut player2: VecDeque<usize> = player2.iter().cloned().collect();
    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
    if player1.is_empty() {
        player2
    } else {
        player1
    }
}

fn score(cards: &VecDeque<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| (idx + 1) * val)
        .sum()
}

struct Game {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

impl Game {
    fn new_from_iter<A: Iterator<Item = usize>, B: Iterator<Item = usize>>(
        player1: A,
        player2: B,
    ) -> Self {
        Self {
            player1: player1.collect(),
            player2: player2.collect(),
        }
    }

    fn new_from_slice(player1: &[usize], player2: &[usize]) -> Self {
        Self::new_from_iter(player1.iter().cloned(), player2.iter().cloned())
    }

    fn play(self) -> FinishedGame {
        let Game {
            mut player1,
            mut player2,
        } = self;
        // let mut seen_cards = HashSet::new();
        let mut seen_rounds = HashSet::new();
        while !player1.is_empty() && !player2.is_empty() {
            // check if this round has already been played
            let player1_cards = player1.iter().cloned().collect::<Vec<_>>();
            let player2_cards = player2.iter().cloned().collect::<Vec<_>>();
            if seen_rounds.contains(&(player1_cards.clone(), player2_cards.clone())) {
                return FinishedGame {
                    won_by_player1: true,
                    winners_hand: player1,
                };
            }
            seen_rounds.insert((player1_cards, player2_cards));

            let p1 = player1.pop_front().unwrap();
            let p2 = player2.pop_front().unwrap();
            let round_won_by_player1 = if p1 <= player1.len() && p2 <= player2.len() {
                let sub_game = Game::new_from_iter(
                    player1.iter().take(p1).cloned(),
                    player2.iter().take(p2).cloned(),
                );
                sub_game.play().won_by_player1
            } else {
                p1 > p2
            };

            if round_won_by_player1 {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        }
        let won_by_player1 = player2.is_empty();
        let winners_hand = if player1.is_empty() { player2 } else { player1 };
        FinishedGame {
            won_by_player1,
            winners_hand,
        }
    }
}

struct FinishedGame {
    won_by_player1: bool,
    winners_hand: VecDeque<usize>,
}

impl FinishedGame {
    fn score(&self) -> usize {
        score(&self.winners_hand)
    }
}

fn main() {
    let (player1, player2) =
        parse_input(include_str!("../../inputs/day22.txt")).expect("input should parse");
    let winners_hand = play(&player1, &player2);
    println!("Part 1: {}", score(&winners_hand));
    println!(
        "Part 2: {}",
        Game::new_from_slice(&player1, &player2).play().score()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let (player1, player2) =
            parse_input(include_str!("../../inputs/day22-sample.txt")).expect("input should parse");
        let winners_hand = play(&player1, &player2);
        assert_eq!(score(&winners_hand), 306);
    }

    #[test]
    fn test_part2_sample() {
        let (player1, player2) =
            parse_input(include_str!("../../inputs/day22-sample.txt")).expect("input should parse");
        let game = Game::new_from_slice(&player1, &player2).play();
        assert!(!game.won_by_player1);
        assert_eq!(
            game.winners_hand.iter().cloned().collect::<Vec<_>>(),
            vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]
        );
        assert_eq!(game.score(), 291);
    }

    #[test]
    fn test_part2_loop() {
        let player1 = &[40, 13, 38, 27, 34, 26, 50, 4, 31];
        let player2 = &[49, 46];
        let game = Game::new_from_slice(player1, player2).play();
        assert!(game.won_by_player1);
    }

    #[test]
    fn test_part2_loop2() {
        let player1 = &[43, 19];
        let player2 = &[2, 29, 14];
        let game = Game::new_from_slice(player1, player2).play();
        assert!(game.won_by_player1);
    }
}

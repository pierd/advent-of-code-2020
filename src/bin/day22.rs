use std::collections::VecDeque;

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

fn main() {
    let (player1, player2) =
        parse_input(include_str!("../../inputs/day22.txt")).expect("input should parse");
    let winners_hand = play(&player1, &player2);
    println!("Part 1: {}", score(&winners_hand));
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
}

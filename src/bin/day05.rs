use std::collections::HashSet;

fn binary_decode(input: &str, low: char, high: char) -> usize {
    input
        .chars()
        .filter(|c| *c == low || *c == high)
        .fold(0, |acc, c| (acc << 1) | if c == high { 1 } else { 0 })
}

fn decode(input: &str) -> (usize, usize) {
    (
        binary_decode(input, 'F', 'B'),
        binary_decode(input, 'L', 'R'),
    )
}

fn seat_id((row, col): (usize, usize)) -> usize {
    row * 8 + col
}

fn find_missing(seats: &[(usize, usize)]) -> Option<usize> {
    let all_seats = seats.iter().cloned().map(seat_id).collect::<HashSet<_>>();
    dbg!(all_seats.len());
    (1..888).find(|&id| {
        !all_seats.contains(&id) && all_seats.contains(&(id - 1)) && all_seats.contains(&(id + 1))
    })
}

fn main() {
    let seats = include_str!("../../inputs/day05.txt")
        .lines()
        .map(decode)
        .collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        seats.clone().into_iter().map(seat_id).max().unwrap()
    );
    println!("Part 2: {}", find_missing(&seats).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode("BFFFBBFRRR"), (70, 7));
        assert_eq!(decode("FFFBBBFRRR"), (14, 7));
        assert_eq!(decode("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id((70, 7)), 567);
        assert_eq!(seat_id((14, 7)), 119);
        assert_eq!(seat_id((102, 4)), 820);
    }
}

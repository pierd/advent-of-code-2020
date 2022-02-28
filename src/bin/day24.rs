use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Coords {
    row: isize,
    col: isize,
}

impl Coords {
    fn move_in_direction(self, dir: Direction) -> Self {
        match dir {
            Direction::East => Self {
                col: self.col + 2,
                ..self
            },
            Direction::SouthEast => Self {
                col: self.col + 1,
                row: self.row - 1,
            },
            Direction::SouthWest => Self {
                col: self.col - 1,
                row: self.row - 1,
            },
            Direction::West => Self {
                col: self.col - 2,
                ..self
            },
            Direction::NorthWest => Self {
                col: self.col - 1,
                row: self.row + 1,
            },
            Direction::NorthEast => Self {
                col: self.col + 1,
                row: self.row + 1,
            },
        }
    }

    fn move_on_path(self, path: &[Direction]) -> Self {
        let mut result = self;
        for dir in path {
            result = result.move_in_direction(*dir);
        }
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::East,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::NorthEast,
];

struct DirectionAdapterIterator<Iter> {
    iter: Iter,
}

impl<Iter> Iterator for DirectionAdapterIterator<Iter>
where
    Iter: Iterator<Item = char>,
{
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some('e') => Some(Direction::East),
            Some('s') => match self.iter.next() {
                Some('e') => Some(Direction::SouthEast),
                Some('w') => Some(Direction::SouthWest),
                _ => None,
            },
            Some('w') => Some(Direction::West),
            Some('n') => match self.iter.next() {
                Some('e') => Some(Direction::NorthEast),
                Some('w') => Some(Direction::NorthWest),
                _ => None,
            },
            _ => None,
        }
    }
}

trait MapDirection<T> {
    fn map_direction(self) -> DirectionAdapterIterator<T>;
}

impl<T> MapDirection<T> for T {
    fn map_direction(self) -> DirectionAdapterIterator<T> {
        DirectionAdapterIterator { iter: self }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| line.chars().map_direction().collect())
        .collect()
}

trait FlipSet<T> {
    fn flip(&mut self, value: T);
}

impl<T> FlipSet<T> for HashSet<T>
where
    T: Hash + Eq,
{
    fn flip(&mut self, value: T) {
        if self.contains(&value) {
            self.remove(&value);
        } else {
            self.insert(value);
        }
    }
}

fn solve_part1(direction_sets: &[Vec<Direction>]) -> HashSet<Coords> {
    let mut black_tiles = HashSet::new();
    for path in direction_sets {
        let coords = Coords::default().move_on_path(path);
        black_tiles.flip(coords);
    }
    black_tiles
}

fn step(black_tiles: HashSet<Coords>) -> HashSet<Coords> {
    let affected_tiles: HashSet<_> = black_tiles
        .iter()
        .flat_map(|coords| {
            ALL_DIRECTIONS
                .iter()
                .map(|dir| coords.move_in_direction(*dir))
        })
        .collect();
    let mut new_black_tiles: HashSet<Coords> = Default::default();
    for coords in affected_tiles {
        let blacks_around = ALL_DIRECTIONS
            .iter()
            .filter(|dir| black_tiles.contains(&coords.move_in_direction(**dir)))
            .count();
        if matches!(
            (black_tiles.contains(&coords), blacks_around),
            (true, 1 | 2) | (false, 2)
        ) {
            new_black_tiles.insert(coords);
        }
    }
    new_black_tiles
}

fn solve_part2(mut black_tiles: HashSet<Coords>) -> HashSet<Coords> {
    for _ in 0..100 {
        black_tiles = step(black_tiles);
    }
    black_tiles
}

fn main() {
    let direction_sets = parse_input(include_str!("../../inputs/day24.txt"));
    let initial_black_tiles = solve_part1(&direction_sets);
    println!("Part 1: {}", initial_black_tiles.len());
    println!("Part 2: {}", solve_part2(initial_black_tiles).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "nwwswee".chars().map_direction().collect::<Vec<_>>(),
            vec![
                Direction::NorthWest,
                Direction::West,
                Direction::SouthWest,
                Direction::East,
                Direction::East
            ],
        )
    }

    #[test]
    fn test_coords_move() {
        assert_eq!(
            Coords::default().move_on_path(&"nwwswee".chars().map_direction().collect::<Vec<_>>()),
            Coords::default()
        );
    }

    #[test]
    fn test_part1_sample() {
        let direction_sets = parse_input(include_str!("../../inputs/day24-sample.txt"));
        assert_eq!(solve_part1(&direction_sets).len(), 10);
    }

    #[test]
    #[ignore]
    fn test_part2_sample() {
        let direction_sets = parse_input(include_str!("../../inputs/day24-sample.txt"));
        assert_eq!(solve_part2(solve_part1(&direction_sets)).len(), 2208);
    }
}

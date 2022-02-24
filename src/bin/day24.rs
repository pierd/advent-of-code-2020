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

#[derive(Default, Debug)]
struct FlipSet<T> {
    set: HashSet<T>,
}

impl<T> FlipSet<T> {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    fn len(&self) -> usize {
        self.set.len()
    }

    fn flip(&mut self, value: T)
    where
        T: Hash + Eq,
    {
        if self.set.contains(&value) {
            self.set.remove(&value);
        } else {
            self.set.insert(value);
        }
    }
}

fn solve_part1(direction_sets: &[Vec<Direction>]) -> usize {
    let mut black_tiles = FlipSet::new();
    for path in direction_sets {
        let coords = Coords::default().move_on_path(path);
        black_tiles.flip(coords);
    }
    black_tiles.len()
}

fn main() {
    let direction_sets = parse_input(include_str!("../../inputs/day24.txt"));
    println!("Part 1: {}", solve_part1(&direction_sets));
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
        assert_eq!(solve_part1(&direction_sets), 10);
    }
}

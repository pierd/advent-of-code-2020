#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    fn from(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Floor),
            'L' => Some(Self::Empty),
            '#' => Some(Self::Occupied),
            _ => None,
        }
    }

    fn is_occupied(&self) -> bool {
        *self == Self::Occupied
    }
}

fn parse_input(input: &str) -> Option<Vec<Vec<Seat>>> {
    input
        .lines()
        .map(|line| line.chars().map(Seat::from).collect())
        .collect()
}

fn is_occupied_at_safe(map: &[Vec<Seat>], row: isize, col: isize) -> bool {
    // this is a bit too complex, just trying if it's possible to have it all done through chaining
    col.try_into()
        .ok()
        .and_then(|col: usize| {
            row.try_into()
                .ok()
                .and_then(|row: usize| map.get(row))
                .and_then(|m| m.get(col))
        })
        .map(|s| s.is_occupied())
        .unwrap_or_default()
}

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_adjacent_occupied(map: &[Vec<Seat>], row: usize, col: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter(|(drow, dcol)| is_occupied_at_safe(map, row as isize + drow, col as isize + dcol))
        .count()
}

fn advance_simple(source: &[Vec<Seat>], target: &mut [Vec<Seat>]) -> bool {
    let mut changed = false;
    for (row_idx, (row, target_row)) in source.iter().zip(target.iter_mut()).enumerate() {
        for (col_idx, (seat, target_seat)) in row.iter().zip(target_row.iter_mut()).enumerate() {
            *target_seat = match seat {
                Seat::Empty if count_adjacent_occupied(source, row_idx, col_idx) == 0 => {
                    changed = true;
                    Seat::Occupied
                }
                Seat::Occupied if count_adjacent_occupied(source, row_idx, col_idx) >= 4 => {
                    changed = true;
                    Seat::Empty
                }
                other => *other,
            }
        }
    }
    changed
}

fn count_visible_occupied(map: &[Vec<Seat>], row: usize, col: usize) -> usize {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let mut visible_occupied = 0;
    for (drow, dcol) in DIRECTIONS {
        let mut r = row as isize + *drow;
        let mut c = col as isize + *dcol;
        while (0..rows).contains(&r) && (0..cols).contains(&c) {
            match map[r as usize][c as usize] {
                Seat::Empty => break,
                Seat::Occupied => {
                    visible_occupied += 1;
                    break;
                }
                _ => {}
            }
            r += *drow;
            c += *dcol;
        }
    }
    visible_occupied
}

fn advance_complex(source: &[Vec<Seat>], target: &mut [Vec<Seat>]) -> bool {
    let mut changed = false;
    for (row_idx, (row, target_row)) in source.iter().zip(target.iter_mut()).enumerate() {
        for (col_idx, (seat, target_seat)) in row.iter().zip(target_row.iter_mut()).enumerate() {
            *target_seat = match seat {
                Seat::Empty if count_visible_occupied(source, row_idx, col_idx) == 0 => {
                    changed = true;
                    Seat::Occupied
                }
                Seat::Occupied if count_visible_occupied(source, row_idx, col_idx) >= 5 => {
                    changed = true;
                    Seat::Empty
                }
                other => *other,
            }
        }
    }
    changed
}

fn advance_until_no_change<F>(map: &[Vec<Seat>], advance_fun: F) -> Vec<Vec<Seat>>
where
    F: Fn(&[Vec<Seat>], &mut [Vec<Seat>]) -> bool,
{
    let mut map1 = map.to_vec();
    let mut map2 = map1.clone();
    while advance_fun(&map1, &mut map2) {
        map1.swap_with_slice(&mut map2);
    }
    map2
}

fn count_occupied(map: Vec<Vec<Seat>>) -> usize {
    map.iter()
        .flat_map(|row| row.iter())
        .filter(|s| s.is_occupied())
        .count()
}

fn main() {
    let map = parse_input(include_str!("../../inputs/day11.txt")).expect("input should parse");
    println!(
        "Part 1: {}",
        count_occupied(advance_until_no_change(&map, advance_simple))
    );
    println!(
        "Part 2: {}",
        count_occupied(advance_until_no_change(&map, advance_complex))
    );
}

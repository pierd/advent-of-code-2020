#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn after_turning(self, dir: Direction) -> Self {
        match (self, dir) {
            (Facing::North, Direction::Left) => Facing::West,
            (Facing::North, Direction::Right) => Facing::East,
            (Facing::South, Direction::Left) => Facing::East,
            (Facing::South, Direction::Right) => Facing::West,
            (Facing::East, Direction::Left) => Facing::North,
            (Facing::East, Direction::Right) => Facing::South,
            (Facing::West, Direction::Left) => Facing::South,
            (Facing::West, Direction::Right) => Facing::North,
        }
    }

    fn after_turning_times(self, dir: Direction, times: usize) -> Self {
        let mut result = self;
        for _ in 0..times {
            result = result.after_turning(dir);
        }
        result
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    Move(Facing),
    Turn(Direction),
    Forward,
}

impl TryFrom<char> for Action {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' => Ok(Self::Move(Facing::North)),
            'S' => Ok(Self::Move(Facing::South)),
            'E' => Ok(Self::Move(Facing::East)),
            'W' => Ok(Self::Move(Facing::West)),
            'L' => Ok(Self::Turn(Direction::Left)),
            'R' => Ok(Self::Turn(Direction::Right)),
            'F' => Ok(Self::Forward),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Ship {
    facing: Facing,
    lat: isize,
    long: isize,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            facing: Facing::East,
            lat: 0,
            long: 0,
        }
    }
}

impl Ship {
    fn act(&mut self, (action, value): &(Action, isize)) {
        match action {
            Action::Move(facing) => self.advance(*facing, *value),
            Action::Turn(dir) => {
                self.facing = self.facing.after_turning_times(*dir, *value as usize / 90)
            }
            Action::Forward => self.advance(self.facing, *value),
        }
    }

    fn act_all(mut self, instructions: &[(Action, isize)]) -> Self {
        for instr in instructions {
            self.act(instr);
        }
        self
    }

    fn advance(&mut self, facing: Facing, distance: isize) {
        match facing {
            Facing::North => self.lat += distance,
            Facing::South => self.lat -= distance,
            Facing::East => self.long += distance,
            Facing::West => self.long -= distance,
        }
    }

    fn distance(&self) -> isize {
        self.lat.abs() + self.long.abs()
    }
}

#[derive(Clone, Copy, Debug)]
struct ShipWithWaypoint {
    lat: isize,
    long: isize,
    wp_lat: isize,
    wp_long: isize,
}

impl Default for ShipWithWaypoint {
    fn default() -> Self {
        Self {
            lat: 0,
            long: 0,
            wp_lat: 1,
            wp_long: 10,
        }
    }
}

impl ShipWithWaypoint {
    fn act(&mut self, (action, value): &(Action, isize)) {
        match action {
            Action::Move(facing) => self.move_wp(*facing, *value),
            Action::Turn(dir) => {
                for _ in 0..(*value / 90) {
                    self.rotate_wp(*dir)
                }
            }
            Action::Forward => {
                self.lat += self.wp_lat * *value;
                self.long += self.wp_long * *value;
            }
        }
    }

    fn act_all(mut self, instructions: &[(Action, isize)]) -> Self {
        for instr in instructions {
            self.act(instr);
        }
        self
    }

    fn move_wp(&mut self, facing: Facing, distance: isize) {
        match facing {
            Facing::North => self.wp_lat += distance,
            Facing::South => self.wp_lat -= distance,
            Facing::East => self.wp_long += distance,
            Facing::West => self.wp_long -= distance,
        }
    }

    fn rotate_wp(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                let lat = self.wp_lat;
                self.wp_lat = self.wp_long;
                self.wp_long = -lat;
            }
            Direction::Right => {
                let lat = self.wp_lat;
                self.wp_lat = -self.wp_long;
                self.wp_long = lat;
            }
        }
    }

    fn distance(&self) -> isize {
        self.lat.abs() + self.long.abs()
    }
}

fn parse_input(input: &str) -> Option<Vec<(Action, isize)>> {
    input
        .lines()
        .map(|line| {
            let (first, rest) = line.split_at(1);
            first
                .chars()
                .next()
                .unwrap()
                .try_into()
                .ok()
                .zip(rest.parse().ok())
        })
        .collect()
}

fn main() {
    let instructions =
        parse_input(include_str!("../../inputs/day12.txt")).expect("input should parse");
    println!(
        "Part 1: {}",
        Ship::default().act_all(&instructions).distance()
    );
    println!(
        "Part 2: {}",
        ShipWithWaypoint::default()
            .act_all(&instructions)
            .distance()
    );
}

use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn bounds_around(val: isize, max: usize) -> RangeInclusive<usize> {
    0isize.max(val - 1).try_into().unwrap()..=(val + 1).min(max as isize).try_into().unwrap()
}
struct Space3d {
    m: Vec<Vec<Vec<bool>>>,
}

impl Space3d {
    fn with_2d_slice(slice: &[Vec<bool>]) -> Self {
        Self {
            m: vec![slice.to_vec()],
        }
    }

    fn get(&self, z: isize, y: isize, x: isize) -> bool {
        (0..self.m.len() as isize).contains(&z)
            && (0..self.m[0].len() as isize).contains(&y)
            && (0..self.m[0][0].len() as isize).contains(&x)
            && self.m[z as usize][y as usize][x as usize]
    }

    fn count_neighbours(&self, z: isize, y: isize, x: isize) -> usize {
        let mut count = 0;
        for z in bounds_around(z, self.m.len() - 1) {
            for y in bounds_around(y, self.m[0].len() - 1) {
                for x in bounds_around(x, self.m[0][0].len() - 1) {
                    if self.m[z][y][x] {
                        count += 1;
                    }
                }
            }
        }
        count - if self.get(z, y, x) { 1 } else { 0 }
    }

    fn advance(&self) -> Self {
        let mut m =
            vec![vec![vec![false; self.m[0][0].len() + 2]; self.m[0].len() + 2]; self.m.len() + 2];
        for z in 0..(self.m.len() as isize + 2) {
            for y in 0..(self.m[0].len() as isize + 2) {
                for x in 0..(self.m[0][0].len() as isize + 2) {
                    let active = self.get(z - 1, y - 1, x - 1);
                    let neighbours = self.count_neighbours(z - 1, y - 1, x - 1);
                    m[z as usize][y as usize][x as usize] =
                        neighbours == 3 || (active && neighbours == 2);
                }
            }
        }
        Self { m }
    }

    fn count(&self) -> usize {
        self.m
            .iter()
            .flat_map(|v| v.iter())
            .flat_map(|v| v.iter())
            .filter(|b| **b)
            .count()
    }
}

struct Space4d {
    m: Vec<Vec<Vec<Vec<bool>>>>,
}

impl Space4d {
    fn with_2d_slice(slice: &[Vec<bool>]) -> Self {
        Self {
            m: vec![vec![slice.to_vec()]],
        }
    }

    fn get(&self, w: isize, z: isize, y: isize, x: isize) -> bool {
        (0..self.m.len() as isize).contains(&w)
            && (0..self.m[0].len() as isize).contains(&z)
            && (0..self.m[0][0].len() as isize).contains(&y)
            && (0..self.m[0][0][0].len() as isize).contains(&x)
            && self.m[w as usize][z as usize][y as usize][x as usize]
    }

    fn count_neighbours(&self, w: isize, z: isize, y: isize, x: isize) -> usize {
        let mut count = 0;
        for w in bounds_around(w, self.m.len() - 1) {
            for z in bounds_around(z, self.m[0].len() - 1) {
                for y in bounds_around(y, self.m[0][0].len() - 1) {
                    for x in bounds_around(x, self.m[0][0][0].len() - 1) {
                        if self.m[w][z][y][x] {
                            count += 1;
                        }
                    }
                }
            }
        }
        count - if self.get(w, z, y, x) { 1 } else { 0 }
    }

    fn advance(&self) -> Self {
        let mut m = vec![
            vec![
                vec![vec![false; self.m[0][0][0].len() + 2]; self.m[0][0].len() + 2];
                self.m[0].len() + 2
            ];
            self.m.len() + 2
        ];
        for w in 0..(self.m.len() as isize + 2) {
            for z in 0..(self.m[0].len() as isize + 2) {
                for y in 0..(self.m[0][0].len() as isize + 2) {
                    for x in 0..(self.m[0][0][0].len() as isize + 2) {
                        let active = self.get(w - 1, z - 1, y - 1, x - 1);
                        let neighbours = self.count_neighbours(w - 1, z - 1, y - 1, x - 1);
                        m[w as usize][z as usize][y as usize][x as usize] =
                            neighbours == 3 || (active && neighbours == 2);
                    }
                }
            }
        }
        Self { m }
    }

    fn count(&self) -> usize {
        self.m
            .iter()
            .flat_map(|v| v.iter())
            .flat_map(|v| v.iter())
            .flat_map(|v| v.iter())
            .filter(|b| **b)
            .count()
    }
}

fn main() {
    let initial_slice = parse_input(include_str!("../../inputs/day17.txt"));
    let mut space = Space3d::with_2d_slice(&initial_slice);
    for _ in 0..6 {
        space = space.advance();
    }
    println!("Part 1: {}", space.count());
    let mut space4d = Space4d::with_2d_slice(&initial_slice);
    for _ in 0..6 {
        space4d = space4d.advance();
    }
    println!("Part 2: {}", space4d.count());
}

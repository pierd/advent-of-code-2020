use core::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
    str::FromStr,
};

use num::integer::Roots;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Id(usize);

impl FromIterator<bool> for Id {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .fold(0, |acc, item| (acc << 1) | if item { 1 } else { 0 }),
        )
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

trait TileId {
    fn tile_id(&self) -> usize;
}

trait EdgeId {
    fn edge_id(&self, edge: Edge) -> Id;
    fn rev_edge_id(&self, edge: Edge) -> Id;
}

trait GetTileData {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn get_tile_field(&self, row: usize, col: usize) -> bool;
}

trait FullTile: TileId + EdgeId + GetTileData {}

impl<T> FullTile for T where T: TileId + EdgeId + GetTileData {}

trait TileWrapping
where
    Self: Sized,
{
    fn flip(self) -> FlippedTile<Self>;
    fn rotate(self) -> RotatedTile<Self>;
}

#[derive(Clone, Debug, Eq)]
struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
}

impl Tile {
    fn new(id: usize, data: Vec<Vec<bool>>) -> Self {
        assert!(!data.is_empty(), "tile data can't be empty");
        assert!(
            data.iter().all(|row| !row.is_empty()),
            "tile data can't be empty"
        );
        Self { id, data }
    }

    fn rotations(&self) -> impl Iterator<Item = TransformedTile<'_>> {
        TransformedTile::create_all(self).into_iter()
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .ok_or(())?
            .strip_prefix("Tile ")
            .ok_or(())?
            .strip_suffix(':')
            .ok_or(())?
            .parse::<usize>()
            .map_err(|_| ())?;
        let data = lines
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Ok(Self::new(id, data))
    }
}

impl TileId for Tile {
    fn tile_id(&self) -> usize {
        self.id
    }
}

impl TileId for &Tile {
    fn tile_id(&self) -> usize {
        (*self).tile_id()
    }
}

impl EdgeId for Tile {
    fn edge_id(&self, edge: Edge) -> Id {
        match edge {
            Edge::Top => self.data[0].iter().cloned().collect(),
            Edge::Bottom => self
                .data
                .last()
                .expect("tile can't be empty")
                .iter()
                .cloned()
                .collect(),
            Edge::Left => self.data.iter().map(|row| row[0]).collect(),
            Edge::Right => self
                .data
                .iter()
                .map(|row| *row.last().expect("tile rows can't be empty"))
                .collect(),
        }
    }

    fn rev_edge_id(&self, edge: Edge) -> Id {
        match edge {
            Edge::Top => self.data[0].iter().rev().cloned().collect(),
            Edge::Bottom => self
                .data
                .last()
                .expect("tile can't be empty")
                .iter()
                .rev()
                .cloned()
                .collect(),
            Edge::Left => self.data.iter().rev().map(|row| row[0]).collect(),
            Edge::Right => self
                .data
                .iter()
                .rev()
                .map(|row| *row.last().expect("tile rows can't be empty"))
                .collect(),
        }
    }
}

impl EdgeId for &Tile {
    fn edge_id(&self, edge: Edge) -> Id {
        (*self).edge_id(edge)
    }

    fn rev_edge_id(&self, edge: Edge) -> Id {
        (*self).rev_edge_id(edge)
    }
}

impl GetTileData for Tile {
    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }

    fn get_tile_field(&self, row: usize, col: usize) -> bool {
        self.data[row][col]
    }
}

impl GetTileData for &Tile {
    fn rows(&self) -> usize {
        (*self).rows()
    }

    fn cols(&self) -> usize {
        (*self).cols()
    }

    fn get_tile_field(&self, row: usize, col: usize) -> bool {
        (*self).get_tile_field(row, col)
    }
}

/// Base tile flipped horizontally
struct FlippedTile<T> {
    tile: T,
}

impl<T> FlippedTile<T> {
    fn wrap(tile: T) -> Self {
        Self { tile }
    }
}

impl<T> Clone for FlippedTile<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            tile: self.tile.clone(),
        }
    }
}

impl<T> TileId for FlippedTile<T>
where
    T: TileId,
{
    fn tile_id(&self) -> usize {
        self.tile.tile_id()
    }
}

impl<T> EdgeId for FlippedTile<T>
where
    T: EdgeId,
{
    fn edge_id(&self, edge: Edge) -> Id {
        match edge {
            Edge::Top | Edge::Bottom => self.tile.rev_edge_id(edge),
            Edge::Left => self.tile.edge_id(Edge::Right),
            Edge::Right => self.tile.edge_id(Edge::Left),
        }
    }

    fn rev_edge_id(&self, edge: Edge) -> Id {
        match edge {
            Edge::Top | Edge::Bottom => self.tile.edge_id(edge),
            Edge::Left => self.tile.rev_edge_id(Edge::Right),
            Edge::Right => self.tile.rev_edge_id(Edge::Left),
        }
    }
}

impl<T> GetTileData for FlippedTile<T>
where
    T: GetTileData,
{
    fn rows(&self) -> usize {
        self.tile.rows()
    }

    fn cols(&self) -> usize {
        self.tile.cols()
    }

    fn get_tile_field(&self, row: usize, col: usize) -> bool {
        self.tile.get_tile_field(row, self.cols() - col - 1)
    }
}

/// Base tile rotated once counter-clockwise
struct RotatedTile<T> {
    tile: T,
}

impl<T> RotatedTile<T> {
    fn wrap(tile: T) -> Self {
        Self { tile }
    }
}

impl<T> Clone for RotatedTile<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            tile: self.tile.clone(),
        }
    }
}

impl<T> TileId for RotatedTile<T>
where
    T: TileId,
{
    fn tile_id(&self) -> usize {
        self.tile.tile_id()
    }
}

impl<T> EdgeId for RotatedTile<T>
where
    T: EdgeId,
{
    fn edge_id(&self, edge: Edge) -> Id {
        match edge {
            Edge::Top => self.tile.edge_id(Edge::Right),
            Edge::Right => self.tile.rev_edge_id(Edge::Bottom),
            Edge::Bottom => self.tile.edge_id(Edge::Left),
            Edge::Left => self.tile.rev_edge_id(Edge::Top),
        }
    }

    fn rev_edge_id(&self, edge: Edge) -> Id {
        match edge {
            Edge::Top => self.tile.rev_edge_id(Edge::Right),
            Edge::Right => self.tile.edge_id(Edge::Bottom),
            Edge::Bottom => self.tile.rev_edge_id(Edge::Left),
            Edge::Left => self.tile.edge_id(Edge::Top),
        }
    }
}

impl<T> GetTileData for RotatedTile<T>
where
    T: GetTileData,
{
    fn rows(&self) -> usize {
        self.tile.cols()
    }

    fn cols(&self) -> usize {
        self.tile.rows()
    }

    fn get_tile_field(&self, row: usize, col: usize) -> bool {
        self.tile.get_tile_field(col, self.rows() - row - 1)
    }
}

impl<T> TileWrapping for T {
    fn flip(self) -> FlippedTile<Self> {
        FlippedTile::wrap(self)
    }

    fn rotate(self) -> RotatedTile<Self> {
        RotatedTile::wrap(self)
    }
}

#[derive(Clone, Copy, Debug)]
struct TransformedTile<'a> {
    tile: &'a Tile,
    /// counter-clockwise rotations (0-3)
    rotations: u8,
    /// is the tile flipped horizontally (mirrored)
    flipped: bool,
}

impl<'a> TransformedTile<'a> {
    fn create_all(tile: &'a Tile) -> [Self; 8] {
        [
            Self {
                tile,
                rotations: 0,
                flipped: false,
            },
            Self {
                tile,
                rotations: 1,
                flipped: false,
            },
            Self {
                tile,
                rotations: 2,
                flipped: false,
            },
            Self {
                tile,
                rotations: 3,
                flipped: false,
            },
            Self {
                tile,
                rotations: 0,
                flipped: true,
            },
            Self {
                tile,
                rotations: 1,
                flipped: true,
            },
            Self {
                tile,
                rotations: 2,
                flipped: true,
            },
            Self {
                tile,
                rotations: 3,
                flipped: true,
            },
        ]
    }

    fn map<F, R>(&self, function: F) -> R
    where
        F: Fn(&dyn FullTile) -> R,
    {
        match (self.flipped, self.rotations % 4) {
            (false, 0) => function(&self.tile),
            (false, 1) => function(&self.tile.rotate()),
            (false, 2) => function(&self.tile.rotate().rotate()),
            (false, 3) => function(&self.tile.rotate().rotate().rotate()),
            (true, 0) => function(&self.tile.flip()),
            (true, 1) => function(&self.tile.flip().rotate()),
            (true, 2) => function(&self.tile.flip().rotate().rotate()),
            (true, 3) => function(&self.tile.flip().rotate().rotate().rotate()),
            _ => unreachable!(),
        }
    }
}

impl<'a> TileId for TransformedTile<'a> {
    fn tile_id(&self) -> usize {
        self.tile.id
    }
}

impl<'a> EdgeId for TransformedTile<'a> {
    fn edge_id(&self, edge: Edge) -> Id {
        self.map(|tile| tile.edge_id(edge))
    }

    fn rev_edge_id(&self, edge: Edge) -> Id {
        self.map(|tile| tile.rev_edge_id(edge))
    }
}

impl<'a> GetTileData for TransformedTile<'a> {
    fn rows(&self) -> usize {
        self.map(|tile| tile.rows())
    }

    fn cols(&self) -> usize {
        self.map(|tile| tile.cols())
    }

    fn get_tile_field(&self, row: usize, col: usize) -> bool {
        self.map(|tile| tile.get_tile_field(row, col))
    }
}

#[derive(Clone, Debug)]
struct Solution<'a> {
    map: Vec<Vec<Option<TransformedTile<'a>>>>,
    used_tiles: HashSet<&'a Tile>,
}

impl<'a> Solution<'a> {
    fn with_size(size: usize) -> Self {
        Self {
            map: vec![vec![None; size]; size],
            used_tiles: Default::default(),
        }
    }

    fn put(&mut self, row: usize, col: usize, rotated_tile: TransformedTile<'a>) -> Result<(), ()> {
        assert!(self.map[row][col].is_none());
        if self.used_tiles.contains(rotated_tile.tile) {
            Err(())
        } else {
            self.map[row][col] = Some(rotated_tile);
            self.used_tiles.insert(rotated_tile.tile);
            Ok(())
        }
    }

    fn discard(&mut self, row: usize, col: usize) {
        let rotated_tile = self.map[row][col].take().unwrap();
        self.used_tiles.remove(rotated_tile.tile);
    }

    fn checksum(&self) -> usize {
        assert!(
            self.map
                .iter()
                .flat_map(|row| row.iter())
                .all(|b| b.is_some()),
            "should be solved by now"
        );
        [
            self.map.first().unwrap().first(),
            self.map.first().unwrap().last(),
            self.map.last().unwrap().first(),
            self.map.last().unwrap().last(),
        ]
        .into_iter()
        .map(|x| x.unwrap().unwrap().tile_id())
        .product()
    }

    fn merge_image(&self) -> Option<Tile> {
        if self
            .map
            .iter()
            .flat_map(|row| row.iter())
            .any(|b| b.is_none())
        {
            return None;
        }

        let map_size = self.map.len();

        // dimensions of a tile to merge
        let part_tile_size = self.map[0][0]
            .map(|t| (t.rows(), t.cols()))
            .expect("already checked above");

        // dimensions of a tile after chopping the edges
        let chopped_tile_size = (part_tile_size.0 - 2, part_tile_size.1 - 2);

        // dimensions of a merged tile
        let full_tile_size = (
            chopped_tile_size.0 * map_size,
            chopped_tile_size.1 * map_size,
        );

        // merge all tiles into one (discarding the edges)
        let mut data = vec![vec![false; full_tile_size.1]; full_tile_size.0];
        for (row_id, row) in self.map.iter().enumerate() {
            for (col_id, tile) in row.iter().enumerate() {
                let tile = tile.expect("already checked above");
                for tile_row_idx in 1..(tile.rows() - 1) {
                    for tile_col_idx in 1..(tile.cols() - 1) {
                        data[row_id * chopped_tile_size.0 + tile_row_idx - 1]
                            [col_id * chopped_tile_size.1 + tile_col_idx - 1] =
                            tile.get_tile_field(tile_row_idx, tile_col_idx);
                    }
                }
            }
        }

        // id is not used for this part
        Some(Tile { data, id: 0 })
    }
}

struct Solver<'a> {
    tiles: &'a [Tile],
    size: usize,
    top_edge_id_to_tile: HashMap<Id, Vec<TransformedTile<'a>>>,
    left_edge_id_to_tile: HashMap<Id, Vec<TransformedTile<'a>>>,
    top_left_edge_id_to_tile: HashMap<(Id, Id), Vec<TransformedTile<'a>>>,
}

impl<'a> Solver<'a> {
    fn with_tiles(tiles: &'a [Tile]) -> Self {
        let size = tiles.len().sqrt();
        assert_eq!(
            size * size,
            tiles.len(),
            "tiles should be able to form a square"
        );
        Self {
            tiles,
            size,
            top_edge_id_to_tile: Self::build_edge_id_to_tile_map(tiles, Edge::Top),
            left_edge_id_to_tile: Self::build_edge_id_to_tile_map(tiles, Edge::Left),
            top_left_edge_id_to_tile: Self::build_double_edge_id_to_tile_map(
                tiles,
                Edge::Top,
                Edge::Left,
            ),
        }
    }

    fn build_edge_id_to_tile_map(
        tiles: &'a [Tile],
        edge: Edge,
    ) -> HashMap<Id, Vec<TransformedTile<'a>>> {
        let mut map: HashMap<Id, Vec<TransformedTile<'a>>> = HashMap::new();
        for tile in tiles {
            for rotated_tile in tile.rotations() {
                map.entry(rotated_tile.edge_id(edge))
                    .or_default()
                    .push(rotated_tile);
            }
        }
        map
    }

    fn build_double_edge_id_to_tile_map(
        tiles: &'a [Tile],
        edge1: Edge,
        edge2: Edge,
    ) -> HashMap<(Id, Id), Vec<TransformedTile<'a>>> {
        let mut map: HashMap<(Id, Id), Vec<TransformedTile<'a>>> = HashMap::new();
        for tile in tiles {
            for rotated_tile in tile.rotations() {
                map.entry((rotated_tile.edge_id(edge1), rotated_tile.edge_id(edge2)))
                    .or_default()
                    .push(rotated_tile);
            }
        }
        map
    }

    fn solve(&'a self) -> Option<Solution<'a>> {
        self.solve_at(0, 0, Solution::with_size(self.size)).ok()
    }

    fn solve_at(
        &'a self,
        mut row: usize,
        mut col: usize,
        mut solution: Solution<'a>,
    ) -> Result<Solution<'a>, Solution<'a>> {
        if col >= self.size {
            row += 1;
            col = 0;
        }
        if row == self.size {
            return Ok(solution);
        }

        let left_edge_id = col.checked_sub(1).map(|left_col| {
            solution.map[row][left_col]
                .expect("tile on the left should have been solved by now")
                .edge_id(Edge::Right)
        });
        let top_edge_id = row.checked_sub(1).map(|up_row| {
            solution.map[up_row][col]
                .expect("tile on the left should have been solved by now")
                .edge_id(Edge::Bottom)
        });

        let mut tiles_to_try_temp_storage = Vec::new();
        let rotated_tiles_to_try = match (top_edge_id, left_edge_id) {
            (None, None) => {
                // FIXME: tiles_to_try_temp_storage is only needed so we can return a borrow from this match
                tiles_to_try_temp_storage
                    .extend(self.tiles.iter().flat_map(|tile| tile.rotations()));
                Some(&tiles_to_try_temp_storage)
            }
            (Some(top_edge_id), None) => self.top_edge_id_to_tile.get(&top_edge_id),
            (None, Some(left_edge_id)) => self.left_edge_id_to_tile.get(&left_edge_id),
            (Some(top_edge_id), Some(left_edge_id)) => self
                .top_left_edge_id_to_tile
                .get(&(top_edge_id, left_edge_id)),
        };

        if let Some(rotated_tiles_to_try) = rotated_tiles_to_try {
            for rotated_tile in rotated_tiles_to_try {
                if solution.put(row, col, *rotated_tile).is_ok() {
                    match self.solve_at(row, col + 1, solution) {
                        Ok(solution) => return Ok(solution),
                        Err(failed_solution) => solution = failed_solution,
                    }
                    solution.discard(row, col);
                }
            }
        }
        Err(solution)
    }
}

fn find_patterns<T: GetTileData>(tile: T, pattern: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let pattern_size = (
        pattern.iter().map(|(i, _)| *i).max().unwrap_or_default() + 1,
        pattern.iter().map(|(_, i)| *i).max().unwrap_or_default() + 1,
    );
    let mut result = Vec::new();
    for row in 0..(tile.rows() - pattern_size.0) {
        for col in 0..(tile.cols() - pattern_size.1) {
            if pattern.iter().all(|(pattern_row, pattern_col)| {
                tile.get_tile_field(row + *pattern_row, col + *pattern_col)
            }) {
                result.push((row, col));
            }
        }
    }
    result
}

fn main() {
    let tiles = include_str!("../../inputs/day20.txt")
        .split("\n\n")
        .map(|s| s.parse::<Tile>())
        .collect::<Result<Vec<_>, _>>()
        .expect("input should parse correctly");
    let solver = Solver::with_tiles(&tiles);
    let solution = solver.solve().expect("there should be a solution");
    println!("Part 1: {}", solution.checksum());

    let pattern: Vec<(usize, usize)> =
        "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   "
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                repeat(row)
                    .zip(line.chars().enumerate())
                    .filter_map(|(row, (col, c))| if c == '#' { Some((row, col)) } else { None })
            })
            .collect();
    let merged = solution.merge_image().expect("should be solved by now");
    let (transformed, best_matches) = merged
        .rotations()
        .map(|transformed| (transformed, find_patterns(transformed, &pattern)))
        .max_by_key(|(_, matches)| matches.len())
        .expect("there should be matches");
    let mut points = HashSet::new();
    for row in 0..transformed.rows() {
        for col in 0..transformed.cols() {
            if transformed.get_tile_field(row, col) {
                points.insert((row, col));
            }
        }
    }
    for (offset_row, offset_col) in best_matches {
        for (row, col) in &pattern {
            points.remove(&(offset_row + *row, offset_col + *col));
        }
    }
    println!("Part 2: {}", points.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tile_2383() -> Tile {
        /*
         *  Tile 2383:
         *  ...#.#.#.#
         *  ..##..####
         *  .##...#...
         *  ..#....#.#
         *  ..#..#.#.#
         *  ##..#.....
         *  .#....##.#
         *  .##...#...
         *  ..#...#.#.
         *  ##.###....
         */
        "Tile 2383:\n...#.#.#.#\n..##..####\n.##...#...\n..#....#.#\n..#..#.#.#\n##..#.....\n.#....##.#\n.##...#...\n..#...#.#.\n##.###....".parse::<Tile>().unwrap()
    }

    #[test]
    fn test_tile_tile_id() {
        assert_eq!(get_tile_2383().tile_id(), 2383);
    }

    #[test]
    fn test_tile_edge_id() {
        let tile = get_tile_2383();

        assert_eq!(tile.edge_id(Edge::Top), Id(0b1010101));
        assert_eq!(tile.rev_edge_id(Edge::Top), Id(0b1010101000));
        assert_eq!(tile.edge_id(Edge::Bottom), Id(0b1101110000));
        assert_eq!(tile.rev_edge_id(Edge::Bottom), Id(0b111011));

        assert_eq!(tile.edge_id(Edge::Left), Id(0b10001));
        assert_eq!(tile.rev_edge_id(Edge::Left), Id(0b1000100000));
        assert_eq!(tile.edge_id(Edge::Right), Id(0b1101101000));
        assert_eq!(tile.rev_edge_id(Edge::Right), Id(0b1011011));
    }

    #[test]
    fn test_tile_edge_id_flipped() {
        /*  not flipped:
         *  Tile 2383:
         *  ...#.#.#.#
         *  ..##..####
         *  .##...#...
         *  ..#....#.#
         *  ..#..#.#.#
         *  ##..#.....
         *  .#....##.#
         *  .##...#...
         *  ..#...#.#.
         *  ##.###....
         */
        let tile = get_tile_2383().flip();

        assert_eq!(tile.edge_id(Edge::Top), Id(0b1010101000));
        assert_eq!(tile.rev_edge_id(Edge::Top), Id(0b1010101));
        assert_eq!(tile.edge_id(Edge::Bottom), Id(0b111011));
        assert_eq!(tile.rev_edge_id(Edge::Bottom), Id(0b1101110000));

        assert_eq!(tile.edge_id(Edge::Left), Id(0b1101101000));
        assert_eq!(tile.rev_edge_id(Edge::Left), Id(0b1011011));
        assert_eq!(tile.edge_id(Edge::Right), Id(0b10001));
        assert_eq!(tile.rev_edge_id(Edge::Right), Id(0b1000100000));
    }

    #[test]
    fn test_tile_edge_id_rotated() {
        /*  not rotated:
         *  Tile 2383:
         *  ...#.#.#.#
         *  ..##..####
         *  .##...#...
         *  ..#....#.#
         *  ..#..#.#.#
         *  ##..#.....
         *  .#....##.#
         *  .##...#...
         *  ..#...#.#.
         *  ##.###....
         */
        let tile = get_tile_2383().rotate();

        assert_eq!(tile.edge_id(Edge::Top), Id(0b1101101000));
        assert_eq!(tile.rev_edge_id(Edge::Top), Id(0b1011011));
        assert_eq!(tile.edge_id(Edge::Bottom), Id(0b10001));
        assert_eq!(tile.rev_edge_id(Edge::Bottom), Id(0b1000100000));

        assert_eq!(tile.edge_id(Edge::Left), Id(0b1010101000));
        assert_eq!(tile.rev_edge_id(Edge::Left), Id(0b1010101));
        assert_eq!(tile.edge_id(Edge::Right), Id(0b111011));
        assert_eq!(tile.rev_edge_id(Edge::Right), Id(0b1101110000));
    }

    #[test]
    fn test_tile_edge_id_after_flip_and_2_rotations() {
        /*
         *  Tile 2383:
         *  ...#.#.#.#
         *  ..##..####
         *  .##...#...
         *  ..#....#.#
         *  ..#..#.#.#
         *  ##..#.....
         *  .#....##.#
         *  .##...#...
         *  ..#...#.#.
         *  ##.###....
         */
        let tile = get_tile_2383().flip().rotate().rotate();

        assert_eq!(tile.edge_id(Edge::Left), Id(0b1000100000));
        assert_eq!(tile.rev_edge_id(Edge::Left), Id(0b10001));
        assert_eq!(tile.edge_id(Edge::Right), Id(0b1011011));
        assert_eq!(tile.rev_edge_id(Edge::Right), Id(0b1101101000));
    }

    fn has_same_edges<T1: EdgeId, T2: EdgeId>(tile1: T1, tile2: T2) {
        for edge in [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            assert_eq!(tile1.edge_id(edge), tile2.edge_id(edge));
            assert_eq!(tile1.rev_edge_id(edge), tile2.rev_edge_id(edge));
        }
    }

    fn has_same_data<T1: GetTileData, T2: GetTileData>(tile1: T1, tile2: T2) {
        assert_eq!(tile1.rows(), tile2.rows());
        assert_eq!(tile1.cols(), tile2.cols());
        for row in 0..tile1.rows() {
            for col in 0..tile1.cols() {
                assert_eq!(
                    tile1.get_tile_field(row, col),
                    tile2.get_tile_field(row, col)
                );
            }
        }
    }

    #[test]
    fn test_flip_is_symmetric() {
        let tile = get_tile_2383();
        let flipped_twice = (&tile).flip().flip();
        has_same_edges(&tile, flipped_twice.clone());
        has_same_data(&tile, flipped_twice);
    }

    #[test]
    fn test_rotate_is_periodic() {
        let tile = get_tile_2383();
        let rotated_4times = (&tile).rotate().rotate().rotate().rotate();
        has_same_edges(&tile, rotated_4times.clone());
        has_same_data(&tile, rotated_4times);
    }

    #[test]
    fn test_flip_rotate_combination() {
        let tile = get_tile_2383();
        has_same_edges(
            (&tile).flip().rotate().rotate(),
            (&tile).rotate().rotate().flip(),
        );
    }

    fn get_tile(tiles: &[Tile], id: usize) -> Option<&Tile> {
        tiles.iter().find(|t| t.tile_id() == id)
    }

    fn get_transformed(
        tiles: &[Tile],
        id: usize,
        flipped: bool,
        rotations: u8,
    ) -> Option<TransformedTile<'_>> {
        get_tile(tiles, id).map(|tile| TransformedTile {
            tile,
            rotations,
            flipped,
        })
    }

    #[test]
    fn test_part1_sample() {
        let tiles = include_str!("../../inputs/day20-sample.txt")
            .split("\n\n")
            .map(|s| s.parse::<Tile>())
            .collect::<Result<Vec<_>, _>>()
            .expect("input should parse correctly");
        assert_eq!(tiles.len(), 9);
        let solver = Solver::with_tiles(&tiles);

        let solved = [[
            get_transformed(&tiles, 1951, true, 2).unwrap(),
            get_transformed(&tiles, 2311, true, 2).unwrap(),
            get_transformed(&tiles, 3079, false, 0).unwrap(),
        ]];
        assert_eq!(
            solved[0][0].edge_id(Edge::Right),
            solved[0][1].edge_id(Edge::Left)
        );
        assert!(!solver
            .left_edge_id_to_tile
            .get(&solved[0][1].edge_id(Edge::Left))
            .unwrap()
            .is_empty());
        assert!(!solver
            .left_edge_id_to_tile
            .get(&solved[0][0].edge_id(Edge::Right))
            .unwrap()
            .is_empty());

        let solution = solver.solve().expect("there should be a solution");
        assert_eq!(solution.checksum(), 20899048083289);
    }

    #[test]
    fn test_part1_sample_solution() {
        let tiles = include_str!("../../inputs/day20-sample.txt")
            .split("\n\n")
            .map(|s| s.parse::<Tile>())
            .collect::<Result<Vec<_>, _>>()
            .expect("input should parse correctly");
        assert_eq!(tiles.len(), 9);

        let mut solution = Solution::with_size(3);
        solution
            .put(0, 0, get_transformed(&tiles, 1951, true, 2).unwrap())
            .unwrap();
        solution
            .put(0, 1, get_transformed(&tiles, 2311, true, 2).unwrap())
            .unwrap();
        solution
            .put(0, 2, get_transformed(&tiles, 3079, false, 0).unwrap())
            .unwrap();
        solution
            .put(1, 0, get_transformed(&tiles, 2729, true, 2).unwrap())
            .unwrap();
        solution
            .put(1, 1, get_transformed(&tiles, 1427, true, 2).unwrap())
            .unwrap();
        solution
            .put(1, 2, get_transformed(&tiles, 2473, true, 3).unwrap())
            .unwrap();
        solution
            .put(2, 0, get_transformed(&tiles, 2971, true, 2).unwrap())
            .unwrap();
        solution
            .put(2, 1, get_transformed(&tiles, 1489, true, 2).unwrap())
            .unwrap();
        solution
            .put(2, 2, get_transformed(&tiles, 1171, true, 0).unwrap())
            .unwrap();
        assert_eq!(solution.checksum(), 20899048083289);
    }

    #[test]
    fn test_part1_tiny() {
        let tiles = "Tile 1:\n##\n##\n\nTile 2:\n#.\n#.\n\nTile 3:\n##\n..\n\nTile 4:\n#.\n.."
            .split("\n\n")
            .map(|s| s.parse::<Tile>())
            .collect::<Result<Vec<_>, _>>()
            .expect("input should parse correctly");
        assert_eq!(tiles.len(), 4);
        let solver = Solver::with_tiles(&tiles);
        let solution = solver.solve().expect("there should be a solution");
        assert_eq!(solution.checksum(), 24);
    }
}

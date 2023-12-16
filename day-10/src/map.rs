use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Error, Result};

type Point = (i64, i64);

const DIRS: [Point; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl From<&Point> for Dir {
    fn from(value: &Point) -> Self {
        match value {
            (1, 0) => Dir::East,
            (-1, 0) => Dir::West,
            (0, 1) => Dir::South,
            (0, -1) => Dir::North,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Starting,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::NorthSouth),
            '-' => Ok(Tile::EastWest),
            'L' => Ok(Tile::NorthEast),
            'J' => Ok(Tile::NorthWest),
            '7' => Ok(Tile::SouthWest),
            'F' => Ok(Tile::SouthEast),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Starting),
            _ => Err(anyhow!("invalid tile: {}", value)),
        }
    }
}

impl Tile {
    fn can_from(&self, dir: &Dir) -> bool {
        matches!(
            (self, dir),
            (Tile::Starting, _)
                | (Tile::NorthSouth, Dir::North | Dir::South)
                | (Tile::EastWest, Dir::East | Dir::West)
                | (Tile::NorthEast, Dir::South | Dir::West)
                | (Tile::NorthWest, Dir::South | Dir::East)
                | (Tile::SouthWest, Dir::North | Dir::East)
                | (Tile::SouthEast, Dir::North | Dir::West)
        )
    }

    fn can_go(&self, dir: &Dir) -> bool {
        matches!(
            (self, dir),
            (Tile::Starting, _)
                | (Tile::NorthSouth, Dir::North | Dir::South)
                | (Tile::EastWest, Dir::East | Dir::West)
                | (Tile::NorthEast, Dir::North | Dir::East)
                | (Tile::NorthWest, Dir::North | Dir::West)
                | (Tile::SouthWest, Dir::South | Dir::West)
                | (Tile::SouthEast, Dir::South | Dir::East)
        )
    }
}

#[derive(Debug)]
pub struct Map {
    pub tiles: HashMap<Point, Tile>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| c.try_into().map(|tile| ((x as i64, y as i64), tile)))
            })
            .collect();

        match tiles {
            Ok(tiles) => Ok(Map { tiles }),
            Err(e) => Err(e),
        }
    }
}

impl Map {
    fn get(&self, point: &Point) -> Option<&Tile> {
        self.tiles.get(point)
    }

    pub fn get_starting(&self) -> Option<(Point, &Tile)> {
        self.tiles
            .iter()
            .find(|(_, t)| *t == &Tile::Starting)
            .map(|(p, t)| (*p, t))
    }

    fn get_surrounds(&self, point: &Point) -> Vec<(Point, &Tile)> {
        DIRS.iter()
            .filter_map(|(x, y)| {
                let delta = (point.0 + x, point.1 + y);
                self.get(&delta).map(|tile| (delta, tile))
            })
            .collect()
    }

    pub fn walk(&self, point: &Point, tile: &Tile, prev: &Point) -> (Point, &Tile) {
        let surrounds = self.get_surrounds(point);
        *surrounds
            .iter()
            .filter(|(p, t)| {
                let diff = (p.0 - point.0, p.1 - point.1);
                let dir = Dir::from(&diff);
                t.can_from(&dir) && tile.can_go(&dir)
            })
            .find(|(p, _)| p != prev)
            .expect("to have a successor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_get() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let map: Map = input.parse()?;
        assert_eq!(map.get(&(0, 0)), Some(&Tile::SouthWest));
        assert_eq!(map.get(&(0, 1)), Some(&Tile::Ground));
        assert_eq!(map.get(&(4, 4)), Some(&Tile::NorthWest));
        assert_eq!(map.get(&(5, 4)), None);
        assert_eq!(map.get(&(4, 5)), None);
        assert_eq!(map.get(&(-1, -1)), None);
        Ok(())
    }

    #[test]
    fn test_map_get_surrounds() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let map: Map = input.parse()?;
        assert_eq!(
            map.get_surrounds(&(0, 0)),
            [((1, 0), &Tile::EastWest), ((0, 1), &Tile::Ground)]
        );
        Ok(())
    }

    #[test]
    fn test_tile_can_go() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let map: Map = input.parse()?;
        let a = map.get(&(0, 0)).unwrap();
        assert!(a.can_go(&Dir::South));
        assert!(a.can_go(&Dir::West));
        assert!(!a.can_go(&Dir::North));
        assert!(!a.can_go(&Dir::East));
        Ok(())
    }

    #[test]
    fn test_tile_can_from() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let map: Map = input.parse()?;
        let a = map.get(&(0, 0)).unwrap();
        assert!(a.can_from(&Dir::North));
        assert!(a.can_from(&Dir::East));
        assert!(!a.can_from(&Dir::South));
        assert!(!a.can_from(&Dir::West));
        Ok(())
    }

    #[test]
    fn test_map_walk() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let map: Map = input.parse()?;
        dbg!(map.get(&(4, 3)));
        dbg!(map.get_surrounds(&(4, 3)));
        assert_eq!(
            map.walk(&(4, 3), &Tile::NorthWest, &(4, 2)),
            ((3, 3), &Tile::EastWest)
        );

        Ok(())
    }
}

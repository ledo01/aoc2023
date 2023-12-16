use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

type Point = (i64, i64);

trait Distance {
    fn distance(&self, other: &Self) -> i64;
}

impl Distance for Point {
    fn distance(&self, other: &Point) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    galaxies: Vec<Point>,
    empty_rows: Vec<i64>,
    empty_cols: Vec<i64>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| (c == '#').then_some((x as i64, y as i64)))
                    .collect::<Vec<_>>()
            })
            .collect();

        let width = s
            .lines()
            .next()
            .ok_or(anyhow!("missing line"))?
            .chars()
            .count();
        let height = s.lines().count();
        let empty_rows = (0..height)
            .filter_map(|row| {
                galaxies
                    .iter()
                    .all(|(_, y)| (*y as usize) != row)
                    .then_some(row as i64)
            })
            .collect();
        let empty_cols = (0..width)
            .filter_map(|col| {
                galaxies
                    .iter()
                    .all(|(x, _)| (*x as usize) != col)
                    .then_some(col as i64)
            })
            .collect();

        Ok(Map {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

impl Map {
    fn distance(&self, a: &Point, b: &Point, exp: i64) -> i64 {
        let min_x = a.0.min(b.0);
        let max_x = a.0.max(b.0);
        let min_y = a.1.min(b.1);
        let max_y = a.1.max(b.1);
        let crossed_rows = self
            .empty_rows
            .iter()
            .filter(|row| (min_y..max_y).contains(row))
            .count() as i64;
        let crossed_cols = self
            .empty_cols
            .iter()
            .filter(|col| (min_x..max_x).contains(col))
            .count() as i64;

        a.distance(b) + (crossed_rows + crossed_cols) * exp
    }

    pub fn get_distances_sum(&self, exp: i64) -> i64 {
        self.galaxies
            .iter()
            .combinations_with_replacement(2)
            .filter_map(|gals| (gals[0] != gals[1]).then_some((gals[0], gals[1])))
            .map(|(a, b)| self.distance(a, b, exp))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_point() {
        let a: Point = (6, 1);
        let b: Point = (10, 5);
        assert_eq!(a.distance(&b), 8);
    }

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_parse_map() -> Result<()> {
        let map: Map = TEST_INPUT.parse()?;
        assert_eq!(
            map,
            Map {
                galaxies: vec![
                    (3, 0,),
                    (7, 1,),
                    (0, 2,),
                    (6, 4,),
                    (1, 5,),
                    (9, 6,),
                    (7, 8,),
                    (0, 9,),
                    (4, 9,),
                ],
                empty_rows: vec![3, 7],
                empty_cols: vec![2, 5, 8]
            }
        );
        Ok(())
    }

    #[test]
    fn test_map_distance() -> Result<()> {
        let map: Map = TEST_INPUT.parse()?;
        assert_eq!(map.distance(&map.galaxies[4], &map.galaxies[8], 1), 9);
        assert_eq!(map.distance(&map.galaxies[0], &map.galaxies[6], 1), 15);
        assert_eq!(map.distance(&map.galaxies[2], &map.galaxies[5], 1), 17);
        assert_eq!(map.distance(&map.galaxies[7], &map.galaxies[8], 1), 5);
        Ok(())
    }

    #[test]
    fn test_map_dist_sum() -> Result<()> {
        let map: Map = TEST_INPUT.parse()?;
        assert_eq!(map.get_distances_sum(1), 374);
        assert_eq!(map.get_distances_sum(10 - 1), 1030);
        assert_eq!(map.get_distances_sum(100 - 1), 8410);

        Ok(())
    }
}

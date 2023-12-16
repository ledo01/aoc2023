use anyhow::{anyhow, Result};
use geo::{Contains, Coord, LineString, Polygon};

use crate::map::Map;

pub fn process(input: &str) -> Result<String> {
    let map: Map = input.parse()?;
    let (starting_point, starting_tile) = map
        .get_starting()
        .ok_or(anyhow!("missing starting point S"))?;

    let mut points = vec![starting_point];
    let mut prev = starting_point;
    let mut current = map.walk(&starting_point, starting_tile, &prev);

    while current.0 != starting_point {
        let new_current = map.walk(&current.0, current.1, &prev);
        prev = current.0;
        current = new_current;
        points.push(prev);
    }

    let poly = Polygon::new(LineString::from(points), vec![]);

    let interiors = map
        .tiles
        .iter()
        .filter(|(p, _)| poly.contains(&Coord { x: p.0, y: p.1 }))
        .count();

    Ok(interiors.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(process(input)?, "10");
        Ok(())
    }
}

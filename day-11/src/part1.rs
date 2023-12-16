use anyhow::Result;

use crate::galaxy::Map;

pub fn process(input: &str) -> Result<String> {
    let map: Map = input.parse()?;
    let dists = map.get_distances_sum(1);
    Ok(dists.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_process() -> Result<()> {
        assert_eq!(process(TEST_INPUT)?, "374");
        Ok(())
    }
}

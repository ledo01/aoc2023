use anyhow::{anyhow, Result};

use crate::map::Map;

pub fn process(input: &str) -> Result<String> {
    let map: Map = input.parse()?;
    let (starting_point, starting_tile) = map
        .get_starting()
        .ok_or(anyhow!("missing starting point S"))?;

    let mut steps = 1;
    let mut prev = starting_point;
    let mut current = map.walk(&starting_point, starting_tile, &prev);

    while current.0 != starting_point {
        let new_current = map.walk(&current.0, current.1, &prev);
        prev = current.0;
        current = new_current;
        steps += 1;
    }

    Ok((steps / 2).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(process(input)?, "8");
        Ok(())
    }
}

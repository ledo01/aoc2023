use anyhow::Result;

use crate::schematic::Schematic;

pub fn process(input: &str) -> Result<String> {
    let schematic: Schematic = input.parse().expect("to be valid schematic");
    let mut ratios = 0;
    for gear in &schematic.gears {
        let mut matches = vec![];
        for part in &schematic.parts {
            if part.points.contains(gear) {
                matches.push(part)
            }
        }

        if matches.len() == 2 {
            ratios += matches[0].value * matches[1].value
        }
    }

    Ok(ratios.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}

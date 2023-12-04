use anyhow::Result;

use crate::schematic::Schematic;

pub fn process(input: &str) -> Result<String> {
    let schematic: Schematic = input.parse().expect("to be valid schematic");
    Ok(schematic
        .parts
        .into_iter()
        .filter(|part| {
            part.points
                .intersection(&schematic.symbols)
                .next()
                .is_some()
        })
        .map(|point| point.value)
        .sum::<u32>()
        .to_string())
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> Result<()> {
        let input = "..*\n.12";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}

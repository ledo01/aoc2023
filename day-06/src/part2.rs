use anyhow::Result;

use crate::race::Race;

fn parse_input(input: &str) -> Race {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("to have the times")
        .strip_prefix("Time:")
        .expect("to start with `Time:`")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distances = lines
        .next()
        .expect("to have the times")
        .strip_prefix("Distance:")
        .expect("to start with `Distance:`")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    Race::new(times, distances)
}

pub fn process(input: &str) -> Result<String> {
    let race = parse_input(input);
    Ok(race.get_n_ways().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input).unwrap(), "71503");
        Ok(())
    }
}

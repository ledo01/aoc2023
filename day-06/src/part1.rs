use anyhow::Result;

use crate::race::Race;

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("to have the times")
        .strip_prefix("Time:")
        .expect("to start with `Time:`")
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());
    let distances = lines
        .next()
        .expect("to have the times")
        .strip_prefix("Distance:")
        .expect("to start with `Distance:`")
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());
    times
        .zip(distances)
        .map(|(time, distance)| Race::new(time, distance))
        .collect()
}

pub fn process(input: &str) -> Result<String> {
    let races = parse_input(input);
    Ok(races
        .iter()
        .map(|r| r.get_n_ways())
        .product::<u64>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input).unwrap(), "288");
        Ok(())
    }
}

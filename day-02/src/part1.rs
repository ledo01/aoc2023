use anyhow::Result;

use crate::game::{Game, Round};

pub fn process(input: &str) -> Result<String> {
    let bag = Round::from((12, 13, 14));
    let total: usize = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .filter(|game: &Game| game.rounds.iter().all(|round| bag.contains(round)))
        .map(|game| game.id)
        .sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}

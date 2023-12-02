use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Default, PartialEq)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<(usize, usize, usize)> for Round {
    fn from(value: (usize, usize, usize)) -> Self {
        Round {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Round, ()> {
        let mut round = Round::default();
        s.split(", ")
            .filter_map(|cube| cube.split_once(' '))
            .for_each(|(value, color)| {
                let value: usize = value.parse().expect("should be a number");
                match color {
                    "red" => round.red = value,
                    "green" => round.green = value,
                    "blue" => round.blue = value,
                    _ => {}
                }
            });
        Ok(round)
    }
}

impl Round {
    fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

#[derive(Debug, Default, PartialEq)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Game, ()> {
        let (raw_game, raw_rounds) = s.split_once(": ").ok_or(())?;
        let id: usize = raw_game
            .strip_prefix("Game ")
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;
        let rounds: Vec<Round> = raw_rounds
            .split("; ")
            .filter_map(|raw_round| raw_round.parse().ok())
            .collect();
        Ok(Game { id, rounds })
    }
}

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
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_round_parse() {
        let input = "3 blue, 4 red, 2 green";
        assert_eq!(Round::from((4, 2, 3)), input.parse().unwrap());
    }

    #[rstest]
    #[case("1 blue", "3 blue", false)]
    #[case("1 blue", "1 blue", true)]
    #[case("1 red", "1 blue", false)]
    #[case("1 red, 1 blue", "1 blue", true)]
    #[case("1 blue", "1 red, 1 blue", false)]
    fn test_round_contains(#[case] a: Round, #[case] b: Round, #[case] matches: bool) {
        assert_eq!(a.contains(&b), matches)
    }

    #[test]
    fn test_game_parse() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            Game {
                id: 1,
                rounds: vec![
                    Round::from((4, 0, 3)),
                    Round::from((1, 2, 6)),
                    Round::from((0, 2, 0))
                ]
            },
            input.parse().unwrap()
        );
    }

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

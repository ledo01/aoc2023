use derive_more::{From, Sum};
use std::cmp::max;
use std::{ops::Add, str::FromStr};

#[derive(Debug, Default, PartialEq, Sum, From)]
pub struct Round {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
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

impl Add<Round> for Round {
    type Output = Round;

    fn add(self, rhs: Round) -> Round {
        Round {
            red: max(self.red, rhs.red),
            green: max(self.green, rhs.green),
            blue: max(self.blue, rhs.blue),
        }
    }
}

impl Round {
    pub fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_round_parse() {
        let input = "3 blue, 4 red, 2 green";
        assert_eq!(Round::from((4, 2, 3)), input.parse().unwrap());
    }

    #[test]
    fn test_round_add() {
        let a = Round::from((2, 1, 1));
        let b = Round::from((1, 1, 2));
        assert_eq!(a + b, Round::from((2, 1, 2)))
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
}

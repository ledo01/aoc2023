use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: usize,
    pub count: usize,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Card, Self::Err> {
        let (card, rest) = s.split_once(": ").expect("to be formated");
        let id = card
            .split_whitespace()
            .nth(1)
            .ok_or("failed to parse card id")?
            .parse::<usize>()
            .map_err(|e| format!("Card id is not a number: {e}"))?;
        let (winnings, numbers) = rest.split_once(" | ").ok_or("Missing | separetor")?;
        let winnings: Vec<u32> = parse_whitespace_spec(winnings);
        let numbers: Vec<u32> = parse_whitespace_spec(numbers);
        let count = numbers.iter().filter(|n| winnings.contains(n)).count();

        Ok(Card { id, count })
    }
}

/// Parse space separated items
fn parse_whitespace_spec<T: FromStr>(s: &str) -> Vec<T> {
    s.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card { id: 1, count: 4 };
        assert_eq!(input.parse::<Card>().unwrap(), card);
    }
}

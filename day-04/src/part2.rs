use anyhow::Result;

use crate::card::Card;

/// Recursivly returs the number of winning cards form the given card.
fn process_card(card: &Card, cards: &[Card]) -> usize {
    let mut res = 1;
    for id in card.id..card.id + card.count {
        res += process_card(&cards[id], cards);
    }
    res
}

pub fn process(input: &str) -> Result<String> {
    let cards: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();
    Ok(cards
        .iter()
        .map(|card| process_card(card, cards.as_slice()))
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_card() {
        let cards = vec![Card::new(1, 2), Card::new(2, 1), Card::new(3, 0)];
        assert_eq!(process_card(&cards[0], cards.as_slice()), 4)
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input).unwrap(), "30");
        Ok(())
    }
}

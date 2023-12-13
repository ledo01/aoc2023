use std::cmp::Ordering;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    N(u32),
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            c => c.to_digit(10).ok_or(()).map(Card::N),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    pub fn new(value: &str) -> Hand {
        let cards: [Card; 5] = value
            .chars()
            .flat_map(|c| c.try_into())
            .collect::<Vec<_>>()
            .try_into()
            .expect("to only have 5 card");
        let counts = cards.iter().counts();
        let counts: Vec<_> = counts.iter().sorted_by(|a, b| b.1.cmp(a.1)).collect();
        let card_type = match counts.len() {
            1 => HandType::FiveOfKind,
            2 if *counts[0].1 == 4 => HandType::FourOfKind,
            2 if *counts[0].1 == 3 => HandType::FullHouse,
            3 if *counts[0].1 == 3 => HandType::ThreeOfKind,
            3 if *counts[0].1 == 2 => HandType::TwoPair,
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Hand {
            cards,
            hand_type: card_type,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Equal => {}
                        ord => return ord,
                    }
                }
            }
            ord => return ord,
        }
        Ordering::Equal
    }
}

pub fn process(input: &str) -> Result<String> {
    let hands: u32 = input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(hand, c)| (Hand::new(hand), c.parse::<u32>().unwrap()))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .enumerate()
        .map(|(idx, (_, val))| (idx as u32 + 1) * val)
        .sum();

    Ok(hands.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('A', Card::A)]
    #[case('K', Card::K)]
    #[case('3', Card::N(3))]
    fn test_parse_card(#[case] value: char, #[case] expected: Card) -> Result<()> {
        let card: Card = value.try_into().unwrap();
        assert_eq!(card, expected);
        Ok(())
    }

    #[rstest]
    #[case("23456", HandType::HighCard)]
    #[case("A23A4", HandType::OnePair)]
    #[case("23432", HandType::TwoPair)]
    #[case("TTT98", HandType::ThreeOfKind)]
    #[case("23332", HandType::FullHouse)]
    #[case("AA8AA", HandType::FourOfKind)]
    #[case("AAAAA", HandType::FiveOfKind)]
    fn test_new_card(#[case] cards: &str, #[case] expected: HandType) {
        let card = Hand::new(cards);
        assert_eq!(card.hand_type, expected);
    }

    #[test]
    fn test_ord() {
        assert!(Hand::new("33332") > Hand::new("2AAAA"));
        assert!(Hand::new("33332") > Hand::new("23456"));
        assert!(Hand::new("33334") > Hand::new("33332"));
        assert!(Hand::new("3333A") > Hand::new("33332"));
        assert!(Hand::new("QQQJA") > Hand::new("T55J5"));
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input)?, "6440");
        Ok(())
    }
}

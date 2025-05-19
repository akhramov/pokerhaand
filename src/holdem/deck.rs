//! Dette modulen representerer en fransk kortstokk
//! med 52 kort.
mod provider;

use core::cmp::Ordering;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use provider::*;

/// Rangering som er uavhengig av farge
#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum Rank {
    /// 2–10, ess representeres med egen variant
    Numeral(usize),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn value(&self) -> usize {
        match self {
            Rank::Numeral(n) => *n,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

/// Farge, fransk standard
#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Representasjon av et kort
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Card {
    pub(crate) suit: Suit,
    pub(crate) rank: Rank,
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rank_str = match self.rank {
            Rank::Numeral(10) => "t".to_string(),
            Rank::Numeral(n) => n.to_string(),
            Rank::Jack => "j".to_string(),
            Rank::Queen => "q".to_string(),
            Rank::King => "k".to_string(),
            Rank::Ace => "a".to_string(),
        };

        let suit_char = match self.suit {
            Suit::Clubs => "k",
            Suit::Diamonds => "r",
            Suit::Hearts => "h",
            Suit::Spades => "s",
        };

        serializer.serialize_str(&format!("{}{}", rank_str, suit_char))
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        if s.len() < 2 || s.len() > 3 {
            return Err(D::Error::custom("Invalid card format"));
        }

        let (rank_str, suit_char) = s.split_at(s.len() - 1);

        let rank = match rank_str {
            "2" => Rank::Numeral(2),
            "3" => Rank::Numeral(3),
            "4" => Rank::Numeral(4),
            "5" => Rank::Numeral(5),
            "6" => Rank::Numeral(6),
            "7" => Rank::Numeral(7),
            "8" => Rank::Numeral(8),
            "9" => Rank::Numeral(9),
            "10" | "t" => Rank::Numeral(10),
            "j" => Rank::Jack,
            "q" => Rank::Queen,
            "k" => Rank::King,
            "a" => Rank::Ace,
            _ => return Err(D::Error::custom("Invalid rank")),
        };

        let suit = match suit_char {
            "k" => Suit::Clubs,
            "r" => Suit::Diamonds,
            "h" => Suit::Hearts,
            "s" => Suit::Spades,
            _ => return Err(D::Error::custom("Invalid suit")),
        };

        Ok(Card { rank, suit })
    }
}

/// Standard fransk kortstokk med 52 kort
pub type Deck = Vec<Card>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting_ranks_works() {
        let mut ranks = vec![
            Rank::King,
            Rank::Numeral(2),
            Rank::Jack,
            Rank::Numeral(10),
            Rank::Ace,
            Rank::Queen,
        ];
        ranks.sort();
        assert_eq!(
            ranks,
            vec![
                Rank::Numeral(2),
                Rank::Numeral(10),
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ]
        );
    }

    #[test]
    fn test_serialize_card() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        let json = serde_json::to_string(&card).unwrap();
        assert_eq!(json, "\"ah\"");
    }

    #[test]
    fn test_deserialize_card() {
        let json = "\"ah\"";
        let card: Card = serde_json::from_str(json).unwrap();
        assert_eq!(
            card,
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts
            }
        );
    }

    #[test]
    fn test_all_card_roundtrips() {
        let ranks = vec![
            Rank::Numeral(2),
            Rank::Numeral(3),
            Rank::Numeral(4),
            Rank::Numeral(5),
            Rank::Numeral(6),
            Rank::Numeral(7),
            Rank::Numeral(8),
            Rank::Numeral(9),
            Rank::Numeral(10),
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];

        let suits = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        for rank in ranks {
            for suit in &suits {
                let card = Card { rank, suit: *suit };
                let json = serde_json::to_string(&card).unwrap();
                let parsed: Card = serde_json::from_str(&json).unwrap();
                assert_eq!(card, parsed);
            }
        }
    }

    #[test]
    fn test_invalid_deserialization() {
        let invalid_inputs = vec!["\"\"", "\"zz\"", "\"12x\"", "\"ab\"", "\"3\""];

        for input in invalid_inputs {
            let result: Result<Card, _> = serde_json::from_str(input);
            assert!(result.is_err(), "Expected error for input: {}", input);
        }
    }
}

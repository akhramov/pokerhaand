//! Representerer en poker hånd, dvs den beste kombinasjonen av fem kort
//! spilleren sitter med.
//! https://en.wikipedia.org/wiki/List_of_poker_hands
use core::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use serde::Serialize;

use super::deck::{Card, Rank};

/// Representerer en rangering av en pokerhånd
#[derive(PartialEq, Eq, Debug, Serialize)]
pub enum RankingCategory {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl RankingCategory {
    // cards må være sortert
    fn from_cards(cards: &[Card; 5]) -> Self {
        let is_straight = cards
            .windows(2)
            .all(|window| window[1].rank.value() - window[0].rank.value() == 1);
        let is_flush = cards.iter().all(|card| card.suit == cards[0].suit);

        let counts = cards.iter().fold(HashMap::new(), |mut acc, card| {
            acc.entry(card.rank)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            acc
        });

        let mut freq: Vec<_> = counts.values().cloned().collect();
        freq.sort_by(|a, b| b.cmp(a)); // Sorter synkende

        match freq.as_slice() {
            _ if is_straight && is_flush => Self::StraightFlush,
            _ if is_flush => Self::Flush,
            _ if is_straight => Self::Straight,
            [4, 1] => Self::FourOfAKind,
            [3, 2] => Self::FullHouse,
            [3, 1, 1] => Self::ThreeOfAKind,
            [2, 2, 1] => Self::TwoPair,
            [2, 1, 1, 1] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

impl Ord for RankingCategory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank_value().cmp(&other.rank_value())
    }
}

impl PartialOrd for RankingCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RankingCategory {
    fn rank_value(&self) -> u8 {
        match self {
            Self::StraightFlush => 9,
            Self::FourOfAKind => 8,
            Self::FullHouse => 7,
            Self::Flush => 6,
            Self::Straight => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
}

/// Representerer en poker hånd, dvs den beste kombinasjonen av fem kort
/// spilleren sitter med.
#[derive(Debug, Eq, Serialize)]
pub struct Hand {
    ranking_category: RankingCategory,
    cards: [Card; 5],
}

impl From<[Card; 5]> for Hand {
    fn from(mut cards: [Card; 5]) -> Self {
        cards.sort_by_key(|card| card.rank);

        Hand {
            ranking_category: RankingCategory::from_cards(&cards),
            cards,
        }
    }
}

impl Hand {
    fn find_rank_with_count(&self, count: usize) -> Rank {
        self.find_all_ranks_with_count(count)[0]
    }

    fn find_all_ranks_with_count(&self, count: usize) -> Vec<Rank> {
        let mut freq = HashMap::new();
        for card in &self.cards {
            *freq.entry(card.rank).or_insert(0) += 1;
        }
        let mut ranks: Vec<Rank> = freq
            .iter()
            .filter(|&(_, &c)| c == count)
            .map(|(&rank, _)| rank)
            .collect();

        ranks.sort_by(|a, b| b.cmp(a)); // høyest først
        ranks
    }

    fn kickers(&self, exclude: &[Rank]) -> Vec<Rank> {
        let exclude_set: HashSet<_> = exclude.iter().cloned().collect();

        let mut kickers: Vec<Rank> = self
            .cards
            .iter()
            .filter_map(|card| {
                if exclude_set.contains(&card.rank) {
                    None
                } else {
                    Some(card.rank)
                }
            })
            .collect();

        kickers.reverse(); // kortene er lav-høy, snu for høyest først
        kickers
    }

    fn compare_n_of_a_kind(&self, other: &Self, group_size: usize) -> Ordering {
        let self_n = self.find_rank_with_count(group_size);
        let other_n = other.find_rank_with_count(group_size);
        match self_n.cmp(&other_n) {
            Ordering::Equal => self.kickers(&[self_n]).cmp(&other.kickers(&[other_n])),
            ord => ord,
        }
    }

    fn compare_same_category(&self, other: &Self) -> Ordering {
        use RankingCategory::*;

        match self.ranking_category {
            Flush | HighCard | StraightFlush | Straight => {
                // sammenlign kort baklengs for høyest først
                self.cards
                    .iter()
                    .rev()
                    .map(|c| c.rank)
                    .cmp(other.cards.iter().rev().map(|c| c.rank))
            }

            FourOfAKind => self.compare_n_of_a_kind(other, 4),

            ThreeOfAKind => self.compare_n_of_a_kind(other, 3),

            OnePair => self.compare_n_of_a_kind(other, 2),

            FullHouse => {
                let self_three = self.find_rank_with_count(3);
                let other_three = other.find_rank_with_count(3);
                match self_three.cmp(&other_three) {
                    Ordering::Equal => {
                        let self_pair = self.find_rank_with_count(2);
                        let other_pair = other.find_rank_with_count(2);
                        self_pair.cmp(&other_pair)
                    }
                    ord => ord,
                }
            }

            TwoPair => {
                let self_pairs = self.find_all_ranks_with_count(2);
                let other_pairs = other.find_all_ranks_with_count(2);

                let ord = self_pairs[0].cmp(&other_pairs[0]);
                if ord != Ordering::Equal {
                    return ord;
                }

                let ord = self_pairs[1].cmp(&other_pairs[1]);
                if ord != Ordering::Equal {
                    return ord;
                }

                self.kickers(&self_pairs).cmp(&other.kickers(&other_pairs))
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.ranking_category == other.ranking_category
            && self.compare_same_category(other) == Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ranking_category.cmp(&other.ranking_category) {
            Ordering::Equal => self.compare_same_category(other),
            ord => ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::deck::{
            Card,
            Rank::*,
            Suit::{self, *},
        },
        *,
    };

    fn c(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    #[test]
    fn detects_straight_flush() {
        let cards = [
            c(Hearts, Numeral(6)),
            c(Hearts, Numeral(7)),
            c(Hearts, Numeral(8)),
            c(Hearts, Numeral(9)),
            c(Hearts, Numeral(10)),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::StraightFlush
        );
    }

    #[test]
    fn detects_four_of_a_kind() {
        let cards = [
            c(Spades, Queen),
            c(Hearts, Queen),
            c(Diamonds, Queen),
            c(Clubs, Queen),
            c(Hearts, Ace),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::FourOfAKind
        );
    }

    #[test]
    fn detects_full_house() {
        let cards = [
            c(Clubs, Numeral(3)),
            c(Spades, Numeral(3)),
            c(Hearts, Numeral(3)),
            c(Clubs, Jack),
            c(Diamonds, Jack),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::FullHouse
        );
    }

    #[test]
    fn detects_flush() {
        let cards = [
            c(Diamonds, Numeral(2)),
            c(Diamonds, Numeral(5)),
            c(Diamonds, Numeral(9)),
            c(Diamonds, Jack),
            c(Diamonds, King),
        ];
        assert_eq!(RankingCategory::from_cards(&cards), RankingCategory::Flush);
    }

    #[test]
    fn detects_straight() {
        let cards = [
            c(Clubs, Numeral(4)),
            c(Hearts, Numeral(5)),
            c(Spades, Numeral(6)),
            c(Diamonds, Numeral(7)),
            c(Hearts, Numeral(8)),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::Straight
        );
    }

    #[test]
    fn detects_three_of_a_kind() {
        let cards = [
            c(Spades, King),
            c(Clubs, King),
            c(Diamonds, King),
            c(Hearts, Numeral(4)),
            c(Hearts, Numeral(6)),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::ThreeOfAKind
        );
    }

    #[test]
    fn detects_two_pair() {
        let cards = [
            c(Spades, Ace),
            c(Clubs, Ace),
            c(Hearts, Numeral(5)),
            c(Diamonds, Numeral(5)),
            c(Clubs, Queen),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::TwoPair
        );
    }

    #[test]
    fn detects_one_pair() {
        let cards = [
            c(Hearts, Numeral(9)),
            c(Diamonds, Numeral(9)),
            c(Spades, Numeral(3)),
            c(Clubs, Jack),
            c(Spades, Queen),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::OnePair
        );
    }

    #[test]
    fn detects_high_card() {
        let cards = [
            c(Spades, Numeral(2)),
            c(Clubs, Numeral(4)),
            c(Diamonds, Numeral(7)),
            c(Hearts, Jack),
            c(Clubs, King),
        ];
        assert_eq!(
            RankingCategory::from_cards(&cards),
            RankingCategory::HighCard
        );
    }

    #[test]
    fn higher_category_wins() {
        let flush = Hand::from([
            c(Hearts, Numeral(2)),
            c(Hearts, Numeral(4)),
            c(Hearts, Numeral(6)),
            c(Hearts, Numeral(8)),
            c(Hearts, Queen),
        ]);

        let straight = Hand::from([
            c(Clubs, Numeral(5)),
            c(Diamonds, Numeral(6)),
            c(Spades, Numeral(7)),
            c(Hearts, Numeral(8)),
            c(Clubs, Numeral(9)),
        ]);

        assert!(flush > straight);
    }

    #[test]
    fn kicker_breaks_tie_for_one_pair() {
        let hand1 = Hand::from([
            c(Hearts, Numeral(9)),
            c(Diamonds, Numeral(9)),
            c(Spades, Numeral(3)),
            c(Clubs, Jack),
            c(Spades, Queen),
        ]);

        let hand2 = Hand::from([
            c(Clubs, Numeral(9)),
            c(Spades, Numeral(9)),
            c(Diamonds, Numeral(3)),
            c(Hearts, Numeral(10)),
            c(Diamonds, Queen),
        ]);

        assert!(hand1 > hand2); // Jack kicker > Ten kicker
    }

    #[test]
    fn same_hand_different_order_is_equal() {
        let hand1 = Hand::from([
            c(Hearts, Ace),
            c(Clubs, King),
            c(Diamonds, Queen),
            c(Spades, Jack),
            c(Hearts, Numeral(10)),
        ]);

        let hand2 = Hand::from([
            c(Hearts, Numeral(10)),
            c(Spades, Jack),
            c(Diamonds, Queen),
            c(Clubs, King),
            c(Hearts, Ace),
        ]);

        assert_eq!(hand1, hand2);
    }

    #[test]
    fn two_pair_with_higher_pair_wins() {
        let hand1 = Hand::from([
            c(Clubs, Numeral(4)),
            c(Diamonds, Numeral(4)),
            c(Spades, Numeral(2)),
            c(Hearts, Numeral(2)),
            c(Clubs, Queen),
        ]);

        let hand2 = Hand::from([
            c(Clubs, Numeral(5)),
            c(Diamonds, Numeral(5)),
            c(Spades, Numeral(2)),
            c(Hearts, Numeral(2)),
            c(Hearts, Queen),
        ]);

        assert!(hand2 > hand1); // 5/2 beats 4/2
    }
}

use core::hash::Hash;

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use super::{Card, Deck, Rank, Suit};

pub const DECK_SIZE: usize = 52;

/// Hjelpetype for å generere kortstokker
pub struct DeckProvider {
    sorted_deck: Deck,
}

impl Default for DeckProvider {
    fn default() -> Self {
        let numbers = (2..=10).map(Rank::Numeral);
        let face_cards = [Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

        let ranks = numbers.chain(face_cards);
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        let sorted_deck = ranks
            .flat_map(|rank| suits.map(|suit| Card { suit, rank }))
            .collect();

        Self { sorted_deck }
    }
}

impl DeckProvider {
    /// Returnerer en kortstokk i en deterministisk rekkefølge
    pub fn get_with_seed<H: Hash>(&self, seed: H) -> Deck {
        let mut rng: ChaCha20Rng = Seeder::from(seed).into_rng();
        let mut deck_copy = self.sorted_deck.clone();

        deck_copy.shuffle(&mut rng);
        deck_copy
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_deck_provider_generates_valid_decks() {
        let deck_provider = DeckProvider::default();
        let deck = deck_provider.get_with_seed(0);

        assert_eq!(deck.len(), DECK_SIZE);
        assert_eq!(
            deck.iter()
                .filter(|card| matches!(
                    card,
                    Card {
                        rank: Rank::Numeral(10),
                        ..
                    }
                ))
                .count(),
            4
        );
    }

    #[test]
    fn test_deck_has_no_duplicates() {
        let deck_provider = DeckProvider::default();
        let deck = deck_provider.get_with_seed("any seed");

        let unique: HashSet<_> = deck.iter().collect();

        assert_eq!(
            unique.len(),
            DECK_SIZE,
            "Kortstokken inneholder duplikater!"
        );
    }

    #[test]
    fn test_deck_provider_randomizations_are_stable() {
        let deck_provider = DeckProvider::default();
        let deck = deck_provider.get_with_seed(1339);

        assert_eq!(
            deck[0],
            Card {
                rank: Rank::Jack,
                suit: Suit::Spades
            }
        );
    }
}

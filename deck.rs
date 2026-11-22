#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let values = [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        let mut cards = Vec::with_capacity(52);
        for suit in &suits {
            for value in &values {
                cards.push(Card {
                    suit: suit.clone(),
                    value: value.clone(),
                });
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        use std::cmp::min;
        // Simple Fisher-Yates shuffle using pseudo-random logic or standard RNG if available.
        // Since we want standard lib without external dependencies if possible, we can use a simple LCG or indices.
        // For simplicity and standard compliance without rand crate, we can implement a basic LCG shuffle or pseudo-randomness.
        let mut seed: u64 = 123456789;
        let len = self.cards.len();
        for i in (1..len).rev() {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let j = (seed as usize) % (i + 1);
            self.cards.swap(i, j);
        }
    }

    pub fn draw(&mut self, count: usize) -> Result<Vec<Card>, String> {
        if count > self.cards.len() {
            return Err("Not enough cards in the deck".to_string());
        }
        let mut drawn = Vec::with_capacity(count);
        for _ in 0..count {
            drawn.push(self.cards.remove(0));
        }
        Ok(drawn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_shuffle() {
        let deck1 = Deck::new();
        let mut deck2 = Deck::new();
        deck2.shuffle();
        assert_ne!(deck1.cards, deck2.cards);
    }

    #[test]
    fn test_draw() {
        let mut deck = Deck::new();
        let drawn = deck.draw(5).unwrap();
        assert_eq!(drawn.len(), 5);
        assert_eq!(deck.cards.len(), 47);
    }

    #[test]
    fn test_draw_too_many() {
        let mut deck = Deck::new();
        let result = deck.draw(53);
        assert!(result.is_err());
    }

    #[test]
    fn test_queen_of_hearts_once() {
        let mut deck = Deck::new();
        deck.shuffle();
        let all_cards = deck.draw(52).unwrap();
        let qh_count = all_cards
            .iter()
            .filter(|c| c.value == Value::Queen && c.suit == Suit::Hearts)
            .count();
        assert_eq!(qh_count, 1);
    }
}

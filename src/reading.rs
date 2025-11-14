use crate::deck::DrawnCard;

#[derive(Clone, PartialEq, Default)]
pub struct Reading {
    cards: Vec<DrawnCard>,
}

impl Reading {
    pub fn empty() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn from_cards(cards: Vec<DrawnCard>) -> Self {
        Self { cards }
    }

    pub fn cards(&self) -> &[DrawnCard] {
        &self.cards
    }

    pub fn has_cards(&self) -> bool {
        !self.cards.is_empty()
    }
}

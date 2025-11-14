//! Tarot deck definitions and helpers.
//!
//! The module is intentionally small and documented so newcomers can follow the
//! data flow:
//! - [`TarotCard`] holds the static card metadata.
//! - [`Deck`] gives us a tiny API to draw random cards without touching the data.
//! - [`DrawCount`] is the user-facing option for how many cards to draw.
//! - [`DrawnCard`] combines a card with its upright/reversed orientation.
//!
//! Everything in here is `Copy`/`Clone`, which keeps the Yew components simple
//! because we can pass data around without borrowing gymnastics.

mod cards;

use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub use cards::CARDS;

/// Basic facts for a tarot card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TarotCard {
    pub slug: &'static str,
    pub name: &'static str,
    pub upright: &'static str,
    pub reversed: &'static str,
    pub keywords: &'static [&'static str],
}

impl TarotCard {
    /// Relative path to the matching `.webp` image inside the `assets/` folder.
    pub fn image_path(&self) -> String {
        format!("assets/{}.webp", self.slug)
    }
}

/// How many cards we want to draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawCount {
    One = 1,
    Three = 3,
    Five = 5,
}

impl DrawCount {
    pub const ALL: [DrawCount; 3] = [DrawCount::One, DrawCount::Three, DrawCount::Five];

    pub fn as_usize(self) -> usize {
        self as usize
    }

    pub fn label(self) -> &'static str {
        match self {
            DrawCount::One => "Single",
            DrawCount::Three => "Three Cards",
            DrawCount::Five => "Five Cards",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            DrawCount::One => "A quick pulse check.",
            DrawCount::Three => "Past · Present · Future.",
            DrawCount::Five => "Deep-dive spread.",
        }
    }
}

/// Upright or reversed orientation for a drawn card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Upright,
    Reversed,
}

impl Orientation {
    pub fn label(self) -> &'static str {
        match self {
            Orientation::Upright => "Upright",
            Orientation::Reversed => "Reversed",
        }
    }

    pub fn meaning(self, card: &TarotCard) -> &'static str {
        match self {
            Orientation::Upright => card.upright,
            Orientation::Reversed => card.reversed,
        }
    }

    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        if rng.gen_bool(0.5) {
            Orientation::Upright
        } else {
            Orientation::Reversed
        }
    }
}

/// A tarot card coupled with its randomly chosen orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawnCard {
    pub card: &'static TarotCard,
    pub orientation: Orientation,
}

impl DrawnCard {
    pub fn name(&self) -> &'static str {
        self.card.name
    }

    pub fn image_path(&self) -> String {
        self.card.image_path()
    }

    pub fn orientation_label(&self) -> &'static str {
        self.orientation.label()
    }

    pub fn meaning(&self) -> &'static str {
        self.orientation.meaning(self.card)
    }

    pub fn keywords(&self) -> &'static [&'static str] {
        self.card.keywords
    }
}

/// Thin wrapper around a slice of cards so we can add helper methods.
#[derive(Debug, Clone, Copy)]
pub struct Deck {
    cards: &'static [TarotCard],
}

impl Deck {
    pub const fn new(cards: &'static [TarotCard]) -> Self {
        Self { cards }
    }

    pub const fn standard() -> Self {
        Self::new(CARDS)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn cards(&self) -> &'static [TarotCard] {
        self.cards
    }

    pub fn draw_random(self, count: DrawCount) -> Vec<DrawnCard> {
        let mut rng = thread_rng();
        let mut indices: Vec<usize> = (0..self.cards.len()).collect();
        indices.shuffle(&mut rng);

        indices
            .into_iter()
            .take(count.as_usize())
            .map(|index| DrawnCard {
                card: &self.cards[index],
                orientation: Orientation::random(&mut rng),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_draws_requested_number_of_cards() {
        let deck = Deck::standard();
        let cards = deck.draw_random(DrawCount::One).expect("draw 1");
        assert_eq!(cards.len(), 1);
    }
}

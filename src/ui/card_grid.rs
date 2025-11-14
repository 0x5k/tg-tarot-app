use yew::prelude::*;

use crate::deck::DrawnCard;

/// Displays the drawn cards in a responsive grid.
#[derive(Properties, PartialEq)]
pub struct CardGridProps {
    pub cards: Vec<DrawnCard>,
    pub reveal: bool,
}

#[function_component(CardGrid)]
pub fn card_grid(props: &CardGridProps) -> Html {
    if props.cards.is_empty() {
        return html! {
            <section class="empty-state">
                <h2>{"Ready for a reading?"}</h2>
                <p>{"Tap the button above to draw from the deck."}</p>
            </section>
        };
    }

    let is_single = props.cards.len() == 1;
    let grid_classes = classes!("cards-grid", is_single.then_some("cards-grid--single"));

    html! {
        <section class={grid_classes}>
            { for props.cards.iter().enumerate().map(|(index, card)| render_card(index, *card, props.reveal)) }
        </section>
    }
}

fn render_card(index: usize, card: DrawnCard, reveal: bool) -> Html {
    let delay_style = format!("transition-delay: {}ms", index * 120);
    let keywords = card.keywords().join(", ");

    html! {
        <div class="card-wrapper">
            <p class="card-title">{ card.name() }</p>
            <article class={classes!("card", reveal.then_some("is-revealed"))} style={delay_style}>
                <div class="card-inner">
                    <div class="card-face card-face--front">
                        <span>{"Tarot Deck"}</span>
                    </div>
                    <div class="card-face card-face--back">
                        <img src={card.image_path()} alt={card.name()} loading="lazy" />
                        <div class="card-copy">
                            <header>
                                <h3>{ card.name() }</h3>
                                <p class="card-orientation">{ card.orientation_label() }</p>
                            </header>
                            <p class="card-meaning">{ card.meaning() }</p>
                            <p class="card-keywords">{ format!("Keywords: {}", keywords) }</p>
                        </div>
                    </div>
                </div>
            </article>
        </div>
    }
}

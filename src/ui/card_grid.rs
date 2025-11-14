use yew::prelude::*;

use crate::deck::{DrawnCard, Orientation};

/// Displays the drawn cards in a responsive grid.
#[derive(Properties, PartialEq)]
pub struct CardGridProps {
    pub cards: Vec<DrawnCard>,
}

#[function_component(CardGrid)]
pub fn card_grid(props: &CardGridProps) -> Html {
    let cards = props.cards.clone();
    let flipped = use_state(Vec::<bool>::new);

    {
        let flipped = flipped.clone();
        let cards = cards.clone();
        use_effect_with(cards, move |cards| {
            flipped.set(vec![false; cards.len()]);
            || ()
        });
    }

    if cards.is_empty() {
        return html! {
            <section class="empty-state">
                <h2>{"Ready for a reading?"}</h2>
                <p>{"Tap the button above to draw from the deck."}</p>
            </section>
        };
    }

    let toggle = {
        let flipped = flipped.clone();
        Callback::from(move |index: usize| {
            flipped.set({
                let mut next = (*flipped).clone();
                if index < next.len() {
                    next[index] = !next[index];
                }
                next
            });
        })
    };

    let is_single = cards.len() == 1;
    let grid_classes = classes!("cards-grid", is_single.then_some("cards-grid--single"));

    html! {
        <section class={grid_classes}>
            { for cards.iter().copied().enumerate().map(|(index, card)| {
                let is_flipped = flipped.get(index).copied().unwrap_or(false);
                let on_click = {
                    let toggle = toggle.clone();
                    Callback::from(move |event: MouseEvent| {
                        event.prevent_default();
                        toggle.emit(index);
                    })
                };
                render_card(index, card, is_flipped, on_click.clone(), toggle.clone())
            }) }
        </section>
    }
}

fn render_card(
    index: usize,
    card: DrawnCard,
    is_flipped: bool,
    on_click: Callback<MouseEvent>,
    toggle: Callback<usize>,
) -> Html {
    let delay_style = format!("transition-delay: {}ms", index * 120);
    let description = describe_card(card);

    html! {
        <div class="card-wrapper">
            <p class="card-title">{ card.name() }</p>
            <article
                class={classes!("card", is_flipped.then_some("is-revealed"))}
                style={delay_style}
                onclick={on_click}
                onkeydown={
                    let toggle = toggle.clone();
                    Callback::from(move |event: KeyboardEvent| {
                        if matches!(event.key().as_str(), "Enter" | " " | "Spacebar") {
                            event.prevent_default();
                            toggle.emit(index);
                        }
                    })
                }
                role="button"
                tabindex="0"
            >
                <div class="card-inner">
                    <div class="card-face card-face--front">
                        <img src={card.image_path()} alt={card.name()} loading="lazy" />
                    </div>
                    <div class="card-face card-face--back">
                        <div class="card-copy">
                            <p class="card-orientation-badge">{ card.orientation_label() }</p>
                            <p class="card-meaning">{ card.meaning() }</p>
                            <p class="card-keywords">
                                { for card.keywords().iter().map(|word| html!{ <span class="keyword-chip">{ word }</span> }) }
                            </p>
                            <p class="card-description">{ description }</p>
                        </div>
                    </div>
                </div>
            </article>
        </div>
    }
}

fn describe_card(card: DrawnCard) -> String {
    let stance = match card.orientation {
        Orientation::Upright => "upright and steady",
        Orientation::Reversed => "reversed and reflective",
    };

    format!(
        "Energy hums through your palms; the card sits {stance}. Breathe into your ribs until they soften, then notice the first symbol that tugs at you. Let it point toward one practical adjustment and one emotional check-in today. Anchor the message by naming it aloud, then tuck the intention into your pocket like a protective talisman.",
        stance = stance,
    )
}

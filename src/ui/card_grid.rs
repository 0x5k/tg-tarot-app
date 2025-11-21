use yew::prelude::*;

use crate::deck::{DrawnCard, Orientation};
use crate::i18n::{CardTranslations, Translations};

/// Displays the drawn cards in a responsive grid.
#[derive(Properties, PartialEq)]
pub struct CardGridProps {
    pub cards: Vec<DrawnCard>,
    pub translations: Translations,
    pub card_translations: CardTranslations,
}

#[function_component(CardGrid)]
pub fn card_grid(props: &CardGridProps) -> Html {
    let cards = &props.cards;
    let t = &props.translations;
    let flipped = use_state(Vec::<bool>::new);

    {
        let flipped = flipped.clone();
        let cards = cards.clone();
        use_effect_with(cards, move |cards| {
            flipped.set(vec![false; cards.len()]);
        });
    }

    if cards.is_empty() {
        return html! {
            <section class="empty-state">
                <h2>{ &t.empty.title }</h2>
                <p>{ &t.empty.subtitle }</p>
            </section>
        };
    }

    let toggle = {
        let flipped = flipped.clone();
        Callback::from(move |index: usize| {
            let mut next = (*flipped).clone();
            if index < next.len() {
                next[index] = !next[index];
            }
            flipped.set(next);
        })
    };

    let is_single = cards.len() == 1;
    let grid_classes = classes!("cards-grid", is_single.then_some("cards-grid--single"));
    let ct = &props.card_translations;

    html! {
        <section class={grid_classes}>
            { for cards.iter().enumerate().map(|(index, &card)| {
                let is_flipped = flipped.get(index).copied().unwrap_or(false);
                let toggle = toggle.clone();
                let onclick = Callback::from(move |_: MouseEvent| toggle.emit(index));
                render_card(index, card, is_flipped, onclick, t, ct)
            }) }
        </section>
    }
}

fn render_card(index: usize, card: DrawnCard, is_flipped: bool, onclick: Callback<MouseEvent>, t: &Translations, ct: &CardTranslations) -> Html {
    let delay_style = format!("transition-delay: {}ms", index * 80);

    let suit_class = detect_suit(card.name());

    let stop_propagation = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    let orientation_label = match card.orientation {
        Orientation::Upright => &t.orientation.upright,
        Orientation::Reversed => &t.orientation.reversed,
    };

    // Get translated meaning and keywords, fallback to static card data
    let (meaning, keywords): (String, Vec<String>) = if let Some(card_t) = ct.get(card.card.slug) {
        let meaning = match card.orientation {
            Orientation::Upright => card_t.upright.clone(),
            Orientation::Reversed => card_t.reversed.clone(),
        };
        (meaning, card_t.keywords.clone())
    } else {
        // Fallback to static card data
        (card.meaning().to_string(), card.keywords().iter().map(|s| s.to_string()).collect())
    };

    html! {
        <div class="card-wrapper">
            <p class="card-title">{ card.full_name() }</p>
            <article
                class={classes!("card", is_flipped.then_some("is-revealed"))}
                style={delay_style}
                {onclick}
            >
                <div class="card-inner">
                    <div class="card-face card-face--front">
                        <img src={card.image_path()} alt={card.name()} loading="lazy" />
                    </div>
                    <div class={classes!("card-face", "card-face--back", suit_class)}>
                        <div class="card-copy" onclick={stop_propagation}>
                            <p class="card-orientation-badge">{ orientation_label }</p>
                            <p class="card-meaning">{ meaning }</p>
                            <div class="card-keywords">
                                { for keywords.iter().map(|word| html!{ <span class="keyword-chip">{ word }</span> }) }
                            </div>
                        </div>
                    </div>
                </div>
            </article>
        </div>
    }
}

fn detect_suit(name: &str) -> &'static str {
    if name.contains("Cups") {
        "suit-cups"
    } else if name.contains("Pentacles") {
        "suit-pentacles"
    } else if name.contains("Swords") {
        "suit-swords"
    } else if name.contains("Wands") {
        "suit-wands"
    } else {
        "suit-major"
    }
}


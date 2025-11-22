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

    // TODO v1: Add flip-back button functionality (had layout issues)
    // Problem: Cards can flip to show text, but clicking text area doesn't flip back
    //          (stop_propagation prevents it, which is needed for text selection/copy)
    // Solution: Add a flip_back callback and button on card back:
    //
    // let flip_back = {
    //     let onclick = onclick.clone();
    //     Callback::from(move |e: MouseEvent| {
    //         e.stop_propagation();
    //         onclick.emit(e);
    //     })
    // };
    //
    // Then add button in card-face--back div:
    // <button class="flip-back-btn" onclick={flip_back} title="Flip back">{ "↩" }</button>

    // ===== FLIP V2 =====
    // Fix: Button inside .card-copy to not affect parent flex layout
    let flip_back_v2 = {
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            onclick.emit(e);
        })
    };
    // ===== END FLIP V2 =====

    // orientation_label removed - already shown in card title (full_name)
    // let orientation_label = match card.orientation {
    //     Orientation::Upright => &t.orientation.upright,
    //     Orientation::Reversed => &t.orientation.reversed,
    // };

    // Get translated meaning, keywords, and name - fallback to static card data
    let (meaning, keywords, card_name): (String, Vec<String>, String) = if let Some(card_t) = ct.get(card.card.slug) {
        let meaning = match card.orientation {
            Orientation::Upright => card_t.upright.clone(),
            Orientation::Reversed => card_t.reversed.clone(),
        };
        let name = card_t.name.clone().unwrap_or_else(|| card.card.name.to_string());
        (meaning, card_t.keywords.clone(), name)
    } else {
        // Fallback to static card data
        (card.meaning().to_string(), card.keywords().iter().map(|s| s.to_string()).collect(), card.card.name.to_string())
    };

    // Build full name with translated "reversed" label
    let full_name = match card.orientation {
        Orientation::Upright => card_name,
        Orientation::Reversed => format!("{} ({})", card_name, &t.orientation.reversed),
    };

    html! {
        <div class="card-wrapper">
            <p class="card-title">{ full_name }</p>
            <article
                class={classes!("card", is_flipped.then_some("is-revealed"))}
                style={delay_style}
                {onclick}
            >
                <div class="card-inner">
                    <div class="card-face card-face--front">
                        <img
                            src={card.image_path()}
                            alt={card.name()}
                            loading="lazy"
                            class={classes!(matches!(card.orientation, Orientation::Reversed).then_some("is-reversed"))}
                        />
                    </div>
                    <div class={classes!("card-face", "card-face--back", suit_class)}>
                        <div class="card-copy" onclick={stop_propagation}>
                            // ===== FLIP V2 BUTTON (top position) =====
                            <button class="flip-back-btn-v2" onclick={flip_back_v2}>{ "↩" }</button>
                            // ===== END FLIP V2 BUTTON =====
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


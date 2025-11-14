use yew::events::PointerEvent;
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
    let swipe_origins = use_state(Vec::<Option<f64>>::new);

    {
        let flipped = flipped.clone();
        let swipe_origins = swipe_origins.clone();
        let cards = cards.clone();
        use_effect_with(cards, move |cards| {
            flipped.set(vec![false; cards.len()]);
            swipe_origins.set(vec![None; cards.len()]);
            || ()
        });
    }

    if cards.is_empty() {
        return html! {
            <section class="empty-state">
                <h2>{"Cards can be REVERSED and UPRIGHT"}</h2>
                <p>{"Tap on card for descriptions."}</p>
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
                render_card(
                    index,
                    card,
                    is_flipped,
                    on_click.clone(),
                    toggle.clone(),
                    swipe_origins.clone(),
                )
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
    swipe_origins: UseStateHandle<Vec<Option<f64>>>,
) -> Html {
    let delay_style = format!("transition-delay: {}ms", index * 120);
    let description = describe_card(card);
    let (on_pointer_down, on_pointer_move, on_pointer_release) =
        build_swipe_handlers(index, toggle.clone(), swipe_origins.clone());

    html! {
        <div class="card-wrapper">
            <p class="card-title">{ card.name() }</p>
            <article
                class={classes!("card", is_flipped.then_some("is-revealed"))}
                style={delay_style}
                onclick={on_click}
                onpointerdown={on_pointer_down}
                onpointermove={on_pointer_move}
                onpointerup={on_pointer_release.clone()}
                onpointercancel={on_pointer_release.clone()}
                onpointerleave={on_pointer_release.clone()}
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

fn build_swipe_handlers(
    index: usize,
    toggle: Callback<usize>,
    swipe_origins: UseStateHandle<Vec<Option<f64>>>,
) -> (
    Callback<PointerEvent>,
    Callback<PointerEvent>,
    Callback<PointerEvent>,
) {
    const THRESHOLD: f64 = 55.0;

    let down = {
        let swipe_origins = swipe_origins.clone();
        Callback::from(move |event: PointerEvent| {
            if !event.is_primary() {
                return;
            }
            let mut next = (*swipe_origins).clone();
            if index >= next.len() {
                next.resize(index + 1, None);
            }
            next[index] = Some(event.client_x() as f64);
            swipe_origins.set(next);
        })
    };

    let move_cb = {
        let swipe_origins = swipe_origins.clone();
        let toggle = toggle.clone();
        Callback::from(move |event: PointerEvent| {
            if !event.is_primary() {
                return;
            }
            if let Some(Some(start)) = swipe_origins.get(index) {
                let delta = event.client_x() as f64 - *start;
                if delta.abs() > THRESHOLD {
                    toggle.emit(index);
                    let mut next = (*swipe_origins).clone();
                    if index < next.len() {
                        next[index] = None;
                    }
                    swipe_origins.set(next);
                }
            }
        })
    };

    let release = {
        Callback::from(move |_event: PointerEvent| {
            let mut next = (*swipe_origins).clone();
            if index < next.len() {
                next[index] = None;
                swipe_origins.set(next);
            }
        })
    };

    (down, move_cb, release)
}

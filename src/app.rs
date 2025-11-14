use gloo::timers::callback::Timeout;
use yew::prelude::*;

use crate::deck::{Deck, DrawCount, DrawnCard};
use crate::telegram::{TelegramTheme, init_web_app, theme_style};
use crate::ui::{CardGrid, DrawControls};

#[derive(Clone, PartialEq)]
struct Reading {
    cards: Vec<DrawnCard>,
    stage: RevealStage,
}

impl Reading {
    fn empty() -> Self {
        Self {
            cards: Vec::new(),
            stage: RevealStage::Idle,
        }
    }

    fn start(cards: Vec<DrawnCard>) -> Self {
        Self {
            cards,
            stage: RevealStage::Dealing,
        }
    }

    fn reveal(current: &Self) -> Self {
        if current.cards.is_empty() {
            Self::empty()
        } else {
            Self {
                cards: current.cards.clone(),
                stage: RevealStage::Revealed,
            }
        }
    }

    fn is_revealed(&self) -> bool {
        matches!(self.stage, RevealStage::Revealed)
    }

    fn is_dealing(&self) -> bool {
        matches!(self.stage, RevealStage::Dealing)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum RevealStage {
    Idle,
    Dealing,
    Revealed,
}

impl Default for Reading {
    fn default() -> Self {
        Self::empty()
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let draw_count = use_state(|| DrawCount::One);
    let reading = use_state(Reading::default);
    let theme = use_state(TelegramTheme::default);

    {
        let theme = theme.clone();
        use_effect(move || {
            theme.set(init_web_app());
            || ()
        });
    }

    let handle_select = {
        let draw_count = draw_count.clone();
        Callback::from(move |count: DrawCount| draw_count.set(count))
    };

    let handle_draw = {
        let draw_count = draw_count.clone();
        let reading = reading.clone();
        Callback::from(move |_| {
            let cards = Deck::standard().draw_random(*draw_count);
            reading.set(Reading::start(cards));

            let reveal_handle = reading.clone();
            Timeout::new(240, move || {
                let next = Reading::reveal(&*reveal_handle);
                reveal_handle.set(next);
            })
            .forget();
        })
    };

    html! {
        <main class="layout" style={theme_style(&*theme)}>
            <header class="hero">
                <h1>{"Telegram Tarot"}</h1>
                <p>{"Draw a single card or a mini spread, right inside Telegram."}</p>
            </header>
            <DrawControls
                selected={*draw_count}
                on_select={handle_select}
                on_draw={handle_draw}
                is_busy={reading.is_dealing()}
            />
            <CardGrid cards={reading.cards.clone()} reveal={reading.is_revealed()} />
            <footer class="helper-text">
                <p>{"Tip: drop your 78 .webp card images into the assets/ folder before running Trunk."}</p>
            </footer>
        </main>
    }
}

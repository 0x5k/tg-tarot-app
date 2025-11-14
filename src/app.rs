use yew::prelude::*;

use crate::deck::{Deck, DrawCount};
use crate::feedback::Feedback;
use crate::reading::Reading;
use crate::telegram::{
    copy_to_clipboard, init_web_app, theme_style, use_back_button, BackButtonState, TelegramSetup,
};
use crate::ui::{CardGrid, DrawControls, StatusBanner};

#[function_component(App)]
pub fn app() -> Html {
    let draw_count = use_state(|| DrawCount::One);
    let reading = use_state(Reading::default);
    let feedback = use_state(Feedback::default);
    let telegram = use_state(TelegramSetup::default);

    {
        let telegram = telegram.clone();
        use_effect_with((), move |_| {
            telegram.set(init_web_app());
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
        let feedback = feedback.clone();
        Callback::from(move |_| match Deck::standard().draw_random(*draw_count) {
            Ok(cards) => {
                reading.set(Reading::from_cards(cards));
                feedback.set(Feedback::default());
            }
            Err(err) => {
                reading.set(Reading::empty());
                feedback.set(Feedback::error(err.to_string()));
            }
        })
    };

    let handle_reset = {
        let reading = reading.clone();
        let feedback = feedback.clone();
        Callback::from(move |_| {
            reading.set(Reading::empty());
            feedback.set(Feedback::status("Cleared reading".to_string()));
        })
    };

    let handle_copy = {
        let reading = reading.clone();
        let feedback = feedback.clone();
        Callback::from(move |_| {
            let cards = reading.cards();
            if cards.is_empty() {
                feedback.set(Feedback::error("Draw cards before copying."));
                return;
            }

            let names: Vec<&str> = cards.iter().map(|card| card.name()).collect();
            let payload = names.join("\n");
            match copy_to_clipboard(&payload) {
                Ok(_) => {
                    let count = names.len();
                    let label = match count {
                        1 => "Copied 1 card name".to_string(),
                        3 => "Copied 3 card names".to_string(),
                        5 => "Copied 5 card names".to_string(),
                        _ => format!("Copied {count} card names"),
                    };
                    feedback.set(Feedback::status(label));
                }
                Err(err) => feedback.set(Feedback::error(err)),
            }
        })
    };

    let has_cards = reading.has_cards();
    let is_telegram = telegram.available;

    use_back_button(
        BackButtonState {
            visible: is_telegram && has_cards,
        },
        handle_reset.clone(),
    );

    html! {
        <main class="layout" style={theme_style(&telegram.theme)}>
            <StatusBanner
                status={feedback.status_text().map(str::to_owned)}
                error={feedback.error_text().map(str::to_owned)}
            />
            <CardGrid cards={reading.cards().to_vec()} />
            <DrawControls
                selected={*draw_count}
                on_select={handle_select}
                on_draw={handle_draw.clone()}
                on_copy={handle_copy}
                can_copy={reading.has_cards()}
            />
        </main>
    }
}

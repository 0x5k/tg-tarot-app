use yew::prelude::*;

use crate::deck::{Deck, DrawCount};
use crate::feedback::Feedback;
use crate::i18n::{format_copied_message, CardTranslations, Language, Translations};
use crate::reading::Reading;
use crate::telegram::{
    copy_to_clipboard, detect_language, init_web_app, theme_style, use_back_button,
    BackButtonState, TelegramSetup,
};
use crate::ui::{CardGrid, DrawControls, StatusBanner};

#[function_component(App)]
pub fn app() -> Html {
    let draw_count = use_state(|| DrawCount::One);
    let reading = use_state(Reading::default);
    let feedback = use_state(Feedback::default);
    let telegram = use_state(TelegramSetup::default);
    let language = use_state(Language::default);
    let controls_collapsed = use_state(|| false);

    // Derive translations from current language
    let translations: Translations = language.load_translations();
    let card_translations: CardTranslations = language.load_card_translations();

    {
        let telegram = telegram.clone();
        let language = language.clone();
        use_effect_with((), move |_| {
            telegram.set(init_web_app());
            language.set(detect_language());
            || ()
        });
    }

    let handle_select = {
        let draw_count = draw_count.clone();
        Callback::from(move |count: DrawCount| draw_count.set(count))
    };

    let handle_language_toggle = {
        let language = language.clone();
        Callback::from(move |_| {
            language.set(language.toggle());
        })
    };

    let handle_controls_toggle = {
        let controls_collapsed = controls_collapsed.clone();
        Callback::from(move |_| {
            controls_collapsed.set(!*controls_collapsed);
        })
    };

    let handle_draw = {
        let draw_count = draw_count.clone();
        let reading = reading.clone();
        let feedback = feedback.clone();
        let controls_collapsed = controls_collapsed.clone();
        Callback::from(move |_| match Deck::standard().draw_random(*draw_count) {
            Ok(cards) => {
                reading.set(Reading::from_cards(cards));
                feedback.set(Feedback::default());
                controls_collapsed.set(true); // Collapse controls after drawing
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
        let t = translations.clone();
        let controls_collapsed = controls_collapsed.clone();
        Callback::from(move |_| {
            reading.set(Reading::empty());
            feedback.set(Feedback::status(t.feedback.cleared.clone()));
            controls_collapsed.set(false); // Expand controls on reset
        })
    };

    let handle_copy = {
        let reading = reading.clone();
        let feedback = feedback.clone();
        let t = translations.clone();
        let ct = card_translations.clone();
        Callback::from(move |_| {
            let cards = reading.cards();
            if cards.is_empty() {
                feedback.set(Feedback::error(t.feedback.draw_first.clone()));
                return;
            }

            // Build translated full names
            let names: Vec<String> = cards
                .iter()
                .map(|card| {
                    let card_name = ct
                        .get(card.card.slug)
                        .and_then(|c| c.name.clone())
                        .unwrap_or_else(|| card.card.name.to_string());

                    match card.orientation {
                        crate::deck::Orientation::Upright => card_name,
                        crate::deck::Orientation::Reversed => {
                            format!("{} ({})", card_name, &t.orientation.reversed)
                        }
                    }
                })
                .collect();

            let payload = names.join("\n");
            match copy_to_clipboard(&payload) {
                Ok(_) => {
                    let msg = format_copied_message(&t, names.len());
                    feedback.set(Feedback::status(msg));
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
            <CardGrid
                cards={reading.cards().to_vec()}
                translations={translations.clone()}
                card_translations={card_translations.clone()}
            />
            <DrawControls
                selected={*draw_count}
                on_select={handle_select}
                on_draw={handle_draw.clone()}
                on_copy={handle_copy}
                on_language_toggle={handle_language_toggle}
                on_toggle_collapse={handle_controls_toggle}
                can_copy={reading.has_cards()}
                collapsed={*controls_collapsed}
                translations={translations.clone()}
            />
        </main>
    }
}

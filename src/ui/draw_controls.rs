use yew::prelude::*;

use crate::deck::DrawCount;
use crate::i18n::Translations;

#[derive(Properties, PartialEq)]
pub struct DrawControlsProps {
    pub selected: DrawCount,
    pub on_select: Callback<DrawCount>,
    pub on_draw: Callback<()>,
    pub on_copy: Callback<()>,
    pub on_language_toggle: Callback<()>,
    pub on_toggle_collapse: Callback<()>,
    pub can_copy: bool,
    pub collapsed: bool,
    pub translations: Translations,
}

#[function_component(DrawControls)]
pub fn draw_controls(props: &DrawControlsProps) -> Html {
    let t = &props.translations;
    let collapsed = props.collapsed;

    let section_classes = classes!(
        "controls",
        collapsed.then_some("controls--collapsed")
    );

    html! {
        <section class={section_classes}>
            // Collapse toggle bar (always visible)
            <button
                type="button"
                class="controls-toggle"
                onclick={props.on_toggle_collapse.reform(|_| ())}
                aria-expanded={(!collapsed).to_string()}
            >
                <span class="controls-toggle-icon">{ if collapsed { "▲" } else { "▼" } }</span>
                <span class="controls-toggle-label">{ if collapsed { &t.buttons.draw } else { &t.buttons.hide } }</span>
            </button>

            // Collapsible content
            <div class="controls-content">
                <div class="toggle-group" role="radiogroup" aria-label="Choose spread size">
                    { for DrawCount::ALL.iter().map(|count| render_toggle(*count, props.selected, &props.on_select, t)) }
                </div>
                <div class="controls-buttons">
                    <button type="button" class="button-primary" onclick={props.on_draw.reform(|_| ())}>
                        { &t.buttons.draw }
                    </button>
                    <button type="button" class="button-secondary" onclick={props.on_copy.reform(|_| ())} disabled={!props.can_copy}>
                        { &t.buttons.copy }
                    </button>
                    <button type="button" class="button-language" onclick={props.on_language_toggle.reform(|_| ())}>
                        { &t.buttons.language }
                    </button>
                </div>
            </div>
        </section>
    }
}

fn render_toggle(count: DrawCount, selected: DrawCount, on_select: &Callback<DrawCount>, t: &Translations) -> Html {
    let is_active = count == selected;
    let on_click = {
        let on_select = on_select.clone();
        Callback::from(move |_| on_select.emit(count))
    };

    let (label, description) = match count {
        DrawCount::One => (&t.spreads.single.label, &t.spreads.single.description),
        DrawCount::Three => (&t.spreads.three.label, &t.spreads.three.description),
        DrawCount::Five => (&t.spreads.five.label, &t.spreads.five.description),
    };

    html! {
        <button
            type="button"
            class={classes!("toggle-button", is_active.then_some("active"))}
            onclick={on_click}
            aria-pressed={is_active.to_string()}
        >
            <span class="toggle-label">{ label }</span>
            <span class="toggle-description">{ description }</span>
        </button>
    }
}

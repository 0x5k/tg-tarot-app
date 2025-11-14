use yew::prelude::*;

use crate::deck::DrawCount;

#[derive(Properties, PartialEq)]
pub struct DrawControlsProps {
    pub selected: DrawCount,
    pub on_select: Callback<DrawCount>,
    pub on_draw: Callback<MouseEvent>,
    pub is_busy: bool,
}

#[function_component(DrawControls)]
pub fn draw_controls(props: &DrawControlsProps) -> Html {
    let DrawControlsProps {
        selected,
        on_select,
        on_draw,
        is_busy,
    } = props;

    let handle_draw = {
        let on_draw = on_draw.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            on_draw.emit(event);
        })
    };

    html! {
        <section class="controls">
            <div class="toggle-group" role="radiogroup" aria-label="Choose draw size">
                { for DrawCount::ALL.iter().map(|count| render_toggle(*count, *selected, on_select)) }
            </div>
            <button
                type="button"
                class="button-primary"
                onclick={handle_draw}
                disabled={*is_busy}
            >
                { if *is_busy { "Drawing..." } else { "Draw cards" } }
            </button>
        </section>
    }
}

fn render_toggle(count: DrawCount, selected: DrawCount, on_select: &Callback<DrawCount>) -> Html {
    let is_active = count == selected;
    let on_click = {
        let on_select = on_select.clone();
        Callback::from(move |_| on_select.emit(count))
    };

    html! {
        <button
            type="button"
            class={classes!("toggle-button", is_active.then_some("active"))}
            onclick={on_click}
            aria-pressed={is_active.to_string()}
        >
            <span class="toggle-label">{ count.label() }</span>
            <span class="toggle-description">{ count.description() }</span>
        </button>
    }
}

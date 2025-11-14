use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatusBannerProps {
    pub status: Option<String>,
    pub error: Option<String>,
}

#[function_component(StatusBanner)]
pub fn status_banner(props: &StatusBannerProps) -> Html {
    let status = props.status.as_ref();
    let error = props.error.as_ref();

    let message = match (status, error) {
        (_, Some(error)) => Some((error, true)),
        (Some(status), None) => Some((status, false)),
        _ => None,
    };

    if let Some((text, is_error)) = message {
        let classes = classes!("status-banner", is_error.then_some("status-banner--error"));
        html! {
            <section class={classes}>
                <p>{ text }</p>
            </section>
        }
    } else {
        Html::default()
    }
}

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use yew::AttrValue;

/// Telegram theme colors that we care about.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TelegramTheme {
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub button_color: Option<String>,
    pub button_text_color: Option<String>,
}

/// Tries to initialise the Telegram WebApp SDK and read theme colours.
///
/// The function is safe to call in a regular browser – it falls back to the
/// default palette when Telegram's global object is missing.
pub fn init_web_app() -> TelegramTheme {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(web_app) = web_app_object() {
            let _ = call_method(&web_app, "expand");
            let _ = call_method(&web_app, "ready");

            return TelegramTheme {
                background_color: read_theme_value(&web_app, "bg_color"),
                text_color: read_theme_value(&web_app, "text_color"),
                button_color: read_theme_value(&web_app, "button_color"),
                button_text_color: read_theme_value(&web_app, "button_text_color"),
            };
        }
    }

    TelegramTheme::default()
}

/// Converts the theme into inline CSS so the main layout matches Telegram.
pub fn theme_style(theme: &TelegramTheme) -> AttrValue {
    let mut styles = Vec::new();

    if let Some(color) = &theme.background_color {
        styles.push(format!("background-color:{}", color));
    }

    if let Some(color) = &theme.text_color {
        styles.push(format!("color:{}", color));
    }

    if let Some(color) = &theme.button_color {
        styles.push(format!("--tg-theme-button-color:{}", color));
    }

    if let Some(color) = &theme.button_text_color {
        styles.push(format!("--tg-theme-button-text-color:{}", color));
    }

    styles.join(";").into()
}

#[cfg(target_arch = "wasm32")]
fn web_app_object() -> Option<JsValue> {
    use js_sys::Reflect;
    let window = web_sys::window()?;
    let telegram = Reflect::get(&window, &JsValue::from_str("Telegram")).ok()?;
    Reflect::get(&telegram, &JsValue::from_str("WebApp")).ok()
}

#[cfg(target_arch = "wasm32")]
fn call_method(target: &JsValue, method: &str) -> Result<(), JsValue> {
    use js_sys::Reflect;
    if let Ok(value) = Reflect::get(target, &JsValue::from_str(method)) {
        if let Some(function) = value.dyn_ref::<js_sys::Function>() {
            function.call0(target).map(|_| ())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn read_theme_value(web_app: &JsValue, field: &str) -> Option<String> {
    use js_sys::Reflect;
    let params = Reflect::get(web_app, &JsValue::from_str("themeParams")).ok()?;
    Reflect::get(&params, &JsValue::from_str(field))
        .ok()
        .and_then(|value| value.as_string())
}

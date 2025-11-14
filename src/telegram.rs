use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use telegram_webapp_sdk::{
    core::{context::TelegramContext, init::try_init_sdk, types::theme_params::TelegramThemeParams},
    webapp::TelegramWebApp,
};

#[cfg(target_arch = "wasm32")]
use gloo::utils::document;
#[cfg(target_arch = "wasm32")]
use telegram_webapp_sdk::webapp::types::EventHandle;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlDocument, HtmlTextAreaElement};

/// Telegram theme colors that we care about.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TelegramTheme {
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub button_color: Option<String>,
    pub button_text_color: Option<String>,
}

#[cfg(target_arch = "wasm32")]
impl From<TelegramThemeParams> for TelegramTheme {
    fn from(params: TelegramThemeParams) -> Self {
        Self {
            background_color: params.bg_color,
            text_color: params.text_color,
            button_color: params.button_color,
            button_text_color: params.button_text_color,
        }
    }
}

/// Declarative state for the Telegram MainButton.
#[derive(Clone, PartialEq)]
pub struct MainButtonState {
    pub text: AttrValue,
    pub visible: bool,
    pub enabled: bool,
    pub loading: bool,
}

impl Default for MainButtonState {
    fn default() -> Self {
        Self {
            text: AttrValue::from("Continue"),
            visible: false,
            enabled: true,
            loading: false,
        }
    }
}

/// Declarative visibility for the Telegram BackButton.
#[derive(Clone, PartialEq, Default)]
pub struct BackButtonState {
    pub visible: bool,
}

/// Tries to initialise the Telegram WebApp SDK and read theme colours.
///
/// The function is safe to call in a regular browser – it falls back to the
/// default palette when Telegram's global object is missing.
pub fn init_web_app() -> TelegramTheme {
    #[cfg(target_arch = "wasm32")]
    {
        match try_init_sdk() {
            Ok(true) => {}
            Ok(false) => return TelegramTheme::default(),
            Err(_) => return TelegramTheme::default(),
        }

        if let Some(app) = TelegramWebApp::instance() {
            let _ = app.expand();
            let _ = app.ready();
        }

        if let Some(theme) = TelegramContext::get(|ctx| ctx.theme_params.clone()) {
            return TelegramTheme::from(theme);
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

/// Synchronises the Telegram MainButton with the provided state and handler.
#[hook]
pub fn use_main_button(state: MainButtonState, on_click: Callback<()>) {
    #[cfg(target_arch = "wasm32")]
    {
        use_effect_with(state, |state| {
            sync_main_button(state);
            || ()
        });

        use_effect_with(on_click, |callback| {
            let guard = register_main_button_handler(callback.clone());
            move || drop(guard)
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (state, on_click);
    }
}

/// Synchronises the Telegram BackButton with the provided state and handler.
#[hook]
pub fn use_back_button(state: BackButtonState, on_click: Callback<()>) {
    #[cfg(target_arch = "wasm32")]
    {
        use_effect_with(state, |state| {
            sync_back_button(state);
            || ()
        });

        use_effect_with(on_click, |callback| {
            let guard = register_back_button_handler(callback.clone());
            move || drop(guard)
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (state, on_click);
    }
}

/// Copies text to the clipboard using either the Telegram SDK or the DOM clipboard.
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        if try_telegram_clipboard(text) {
            return Ok(());
        }
        dom_copy(text)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = text;
        Err("Clipboard is only available in a browser context.".into())
    }
}

#[cfg(target_arch = "wasm32")]
fn try_telegram_clipboard(text: &str) -> bool {
    use js_sys::Reflect;
    use wasm_bindgen::JsValue;

    if let Some(web_app) = web_app_object() {
        if let Ok(set_clipboard) = Reflect::get(&web_app, &JsValue::from_str("setClipboardText")) {
            if let Some(function) = set_clipboard.dyn_ref::<js_sys::Function>() {
                let _ = function.call1(&web_app, &JsValue::from_str(text));
                return true;
            }
        }
    }

    false
}

#[cfg(target_arch = "wasm32")]
fn dom_copy(text: &str) -> Result<(), String> {
    let document = document();
    let html_document = document
        .clone()
        .dyn_into::<HtmlDocument>()
        .map_err(|_| "Unable to access HTML document".to_string())?;
    let textarea = document
        .create_element("textarea")
        .map_err(|_| "Unable to access document".to_string())?
        .dyn_into::<HtmlTextAreaElement>()
        .map_err(|_| "Unable to create textarea".to_string())?;
    textarea.set_value(text);
    textarea
        .set_attribute("readonly", "")
        .map_err(|_| "Unable to configure textarea".to_string())?;
    textarea.style().set_property("position", "absolute").ok();
    textarea.style().set_property("left", "-9999px").ok();

    let body = document
        .body()
        .ok_or_else(|| "Unable to access document body".to_string())?;
    body.append_child(&textarea)
        .map_err(|_| "Unable to append helper".to_string())?;
    textarea.select();
    let success = html_document
        .exec_command("copy")
        .map_err(|_| "Clipboard command failed".to_string())?;
    body.remove_child(&textarea).ok();

    if success {
        Ok(())
    } else {
        Err("Clipboard command rejected".into())
    }
}

#[cfg(target_arch = "wasm32")]
fn web_app_object() -> Option<wasm_bindgen::JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsValue;
    let window = web_sys::window()?;
    let telegram = Reflect::get(&window, &JsValue::from_str("Telegram")).ok()?;
    Reflect::get(&telegram, &JsValue::from_str("WebApp")).ok()
}

#[cfg(target_arch = "wasm32")]
fn sync_main_button(state: &MainButtonState) {
    if let Some(app) = TelegramWebApp::instance() {
        let text = state.text.to_string();
        let _ = app.set_main_button_text(&text);

        if state.visible {
            let _ = app.show_main_button();
        } else {
            let _ = app.hide_main_button();
        }

        if state.enabled {
            let _ = app.enable_main_button();
        } else {
            let _ = app.disable_main_button();
        }

        if state.loading {
            let _ = app.show_main_button_progress(false);
        } else {
            let _ = app.hide_main_button_progress();
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn sync_back_button(state: &BackButtonState) {
    if let Some(app) = TelegramWebApp::instance() {
        if state.visible {
            let _ = app.show_back_button();
        } else {
            let _ = app.hide_back_button();
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn register_main_button_handler(callback: Callback<()>) -> Option<EventHandle<dyn FnMut()>> {
    let app = TelegramWebApp::instance()?;
    app.set_main_button_callback(move || callback.emit(())).ok()
}

#[cfg(target_arch = "wasm32")]
fn register_back_button_handler(callback: Callback<()>) -> Option<EventHandle<dyn FnMut()>> {
    let app = TelegramWebApp::instance()?;
    app.set_back_button_callback(move || callback.emit(())).ok()
}

use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo::utils::document;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::Closure;
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
fn call_method(target: &wasm_bindgen::JsValue, method: &str) -> Result<(), wasm_bindgen::JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::{JsCast, JsValue};
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
fn call_method_with_value(
    target: &wasm_bindgen::JsValue,
    method: &str,
    value: &wasm_bindgen::JsValue,
) -> Result<(), wasm_bindgen::JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::{JsCast, JsValue};
    if let Ok(invocable) = Reflect::get(target, &JsValue::from_str(method)) {
        if let Some(function) = invocable.dyn_ref::<js_sys::Function>() {
            function.call1(target, value).map(|_| ())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn read_theme_value(web_app: &wasm_bindgen::JsValue, field: &str) -> Option<String> {
    use js_sys::Reflect;
    use wasm_bindgen::JsValue;
    let params = Reflect::get(web_app, &JsValue::from_str("themeParams")).ok()?;
    Reflect::get(&params, &JsValue::from_str(field))
        .ok()
        .and_then(|value| value.as_string())
}

#[cfg(target_arch = "wasm32")]
fn sync_main_button(state: &MainButtonState) {
    use wasm_bindgen::JsValue;
    if let Some(button) = main_button_object() {
        let text = JsValue::from(state.text.to_string());
        let _ = call_method_with_value(&button, "setText", &text);

        if state.visible {
            let _ = call_method(&button, "show");
        } else {
            let _ = call_method(&button, "hide");
        }

        if state.enabled {
            let _ = call_method(&button, "enable");
        } else {
            let _ = call_method(&button, "disable");
        }

        if state.loading {
            let _ = call_method(&button, "showProgress");
        } else {
            let _ = call_method(&button, "hideProgress");
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn sync_back_button(state: &BackButtonState) {
    if let Some(button) = back_button_object() {
        if state.visible {
            let _ = call_method(&button, "show");
        } else {
            let _ = call_method(&button, "hide");
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn register_main_button_handler(callback: Callback<()>) -> Option<TelegramEventGuard> {
    let button = main_button_object()?;
    let closure = Closure::wrap(Box::new(move || callback.emit(())) as Box<dyn FnMut()>);
    let _ = call_method_with_value(&button, "onClick", closure.as_ref());
    Some(TelegramEventGuard::new(button, "offClick", closure))
}

#[cfg(target_arch = "wasm32")]
fn register_back_button_handler(callback: Callback<()>) -> Option<TelegramEventGuard> {
    let button = back_button_object()?;
    let closure = Closure::wrap(Box::new(move || callback.emit(())) as Box<dyn FnMut()>);
    let _ = call_method_with_value(&button, "onClick", closure.as_ref());
    Some(TelegramEventGuard::new(button, "offClick", closure))
}

#[cfg(target_arch = "wasm32")]
fn main_button_object() -> Option<wasm_bindgen::JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsValue;
    let web_app = web_app_object()?;
    Reflect::get(&web_app, &JsValue::from_str("MainButton")).ok()
}

#[cfg(target_arch = "wasm32")]
fn back_button_object() -> Option<wasm_bindgen::JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsValue;
    let web_app = web_app_object()?;
    Reflect::get(&web_app, &JsValue::from_str("BackButton")).ok()
}

#[cfg(target_arch = "wasm32")]
struct TelegramEventGuard {
    target: wasm_bindgen::JsValue,
    method: &'static str,
    callback: Closure<dyn FnMut()>,
}

#[cfg(target_arch = "wasm32")]
impl TelegramEventGuard {
    fn new(
        target: wasm_bindgen::JsValue,
        method: &'static str,
        callback: Closure<dyn FnMut()>,
    ) -> Self {
        Self {
            target,
            method,
            callback,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for TelegramEventGuard {
    fn drop(&mut self) {
        let _ = call_method_with_value(&self.target, self.method, self.callback.as_ref());
    }
}

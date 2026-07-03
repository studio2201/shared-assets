//! Reusable language switcher Yew component.
//!
//! Renders a `<select>` with all 8 supported languages.
//! On change, writes the `lang` cookie via `set_saved_locale()`
//! then fires the `on_change` callback.

use shared_core::i18n::Language;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::locale::set_saved_locale;

/// Props for [`LanguageSwitcher`].
#[derive(Properties, PartialEq, Clone)]
pub struct LanguageSwitcherProps {
    /// Currently selected language code (e.g. "en", "zh").
    pub current: String,
    /// Called with the new language code when the user picks one.
    pub on_change: Callback<String>,
    /// Optional CSS class for the outer `<select>` element.
    #[prop_or_default]
    pub class: Option<String>,
    /// Optional ARIA label override.
    #[prop_or_default]
    pub aria_label: Option<String>,
}

/// Drop-in `<select>` for picking the active UI language.
#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &LanguageSwitcherProps) -> Html {
    let on_change = {
        let on_change_cb = props.on_change.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                .map(|s| s.value());
            if let Some(lang) = value {
                set_saved_locale(&lang);
                on_change_cb.emit(lang);
            }
        })
    };

    let class = props
        .class
        .clone()
        .unwrap_or_else(|| "language-select".to_string());
    let aria = props
        .aria_label
        .clone()
        .unwrap_or_else(|| "Select language".to_string());

    html! {
        <select class={class} aria-label={aria} onchange={on_change}>
            { for Language::all().iter().map(|lang| {
                let code = lang.code();
                html! {
                    <option value={code} selected={props.current == code}>
                        { lang.label() }
                    </option>
                }
            }) }
        </select>
    }
}

//! Browser locale detection and persistence.
//!
//! Used by every companion app's Header to remember the user's
//! language preference across reloads. Pure Rust (no Yew deps) so
//! both Yew apps and Leptos apps (aura) can call it directly.

/// Returns the user's preferred language code by reading the `lang`
/// cookie first, then falling back to `navigator.language`. Returns
/// `"en"` if neither is set.
pub fn detect_browser_locale() -> String {
    if let Some(saved) = get_saved_locale() {
        return saved;
    }
    let raw = web_sys::window()
        .and_then(|w| w.navigator().language())
        .unwrap_or_default();
    raw.get(..2).unwrap_or("en").to_string()
}

/// Reads the `lang` cookie. Returns `None` if not set.
pub fn get_saved_locale() -> Option<String> {
    use wasm_bindgen::JsCast;
    let document = web_sys::window()?.document()?;
    let html: &web_sys::HtmlDocument = document.dyn_ref()?;
    let raw = html.cookie().ok()?;
    for part in raw.split(';') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix("lang=") {
            return Some(value.trim_matches('"').to_string());
        }
    }
    None
}

/// Writes the `lang` cookie: `lang=<locale>; Path=/; SameSite=Lax`.
pub fn set_saved_locale(locale: &str) {
    use wasm_bindgen::JsCast;
    if let Some(document) = web_sys::window().and_then(|w| w.document())
        && let Ok(html) = document.dyn_into::<web_sys::HtmlDocument>()
    {
        let _ = html.set_cookie(&format!("lang={locale}; Path=/; SameSite=Lax"));
    }
}

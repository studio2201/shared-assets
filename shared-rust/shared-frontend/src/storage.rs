//! LocalStorage helper that mirrors the `"theme"` key to a cookie.
//!
//! Used by every companion app's `Header` to persist the active theme.
//! Snake uses a custom cookie name (`snake_theme`); other apps use the
//! default (`super_metroid_theme`).

/// Default name of the cookie used to mirror the active theme.
const DEFAULT_COOKIE_NAME: &str = "super_metroid_theme";

/// Static facade over the browser storage APIs used by the application.
pub struct StorageService;

impl Default for StorageService {
    fn default() -> Self {
        Self
    }
}

impl StorageService {
    /// Creates a new [`StorageService`].
    pub fn new() -> Self {
        Self
    }

    /// Returns the [`web_sys::Storage`] handle for the current window, if any.
    fn local_storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    /// Reads the raw `document.cookie` string. Returns `None` when the cookie
    /// API is unavailable or when `document.cookie` is not a string.
    fn get_cookie_str() -> Option<String> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let val =
            js_sys::Reflect::get(&document, &wasm_bindgen::JsValue::from_str("cookie")).ok()?;
        val.as_string()
    }

    /// Writes a new cookie string to `document.cookie`. Returns `None` only
    /// when the DOM itself is unavailable; the actual `set` is fire-and-forget.
    fn set_cookie_str(cookie_value: &str) -> Option<()> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let _ = js_sys::Reflect::set(
            &document,
            &wasm_bindgen::JsValue::from_str("cookie"),
            &wasm_bindgen::JsValue::from_str(cookie_value),
        );
        Some(())
    }

    /// Reads a string value by key. For `key == "theme"`, the cookie is
    /// consulted first so the value survives a `localStorage` clear. Returns
    /// an empty string when nothing is set.
    pub fn get_item(&self, key: &str) -> String {
        if key == "theme"
            && let Some(cookie_str) = Self::get_cookie_str()
        {
            for cookie in cookie_str.split(';') {
                let parts: Vec<&str> = cookie.split('=').map(|s| s.trim()).collect();
                if parts.len() >= 2 && parts[0] == DEFAULT_COOKIE_NAME {
                    let val = parts[1].to_string();
                    let clean = if val.starts_with('"') && val.ends_with('"') && val.len() >= 2 {
                        val[1..val.len() - 1].to_string()
                    } else {
                        val
                    };
                    let _ = Self::local_storage().map(|s| s.set_item(key, &clean));
                    return clean;
                }
            }
        }
        match Self::local_storage().and_then(|s| s.get_item(key).ok().flatten()) {
            Some(v) => {
                if v.starts_with('"') && v.ends_with('"') && v.len() >= 2 {
                    let clean = v[1..v.len() - 1].to_string();
                    self.set_item(key, &clean);
                    clean
                } else {
                    v
                }
            }
            None => String::new(),
        }
    }

    /// Writes a string value by key. When `key == "theme"`, the value is also
    /// mirrored to a long-lived cookie so it survives a browser session reset.
    pub fn set_item(&self, key: &str, value: &str) {
        if let Some(s) = Self::local_storage() {
            let _ = s.set_item(key, value);
        }
        if key == "theme" {
            let cookie_value = format!(
                "{DEFAULT_COOKIE_NAME}={}; Path=/; Max-Age=31536000; SameSite=Lax",
                value
            );
            Self::set_cookie_str(&cookie_value);
        }
    }
}

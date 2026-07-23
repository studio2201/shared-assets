//! Reusable numeric-PIN login component.
//!
//! Renders a single numeric input that auto-submits once it reaches the
//! configured length, or that the user can submit explicitly via Enter.
//! The parent handles the API call (emit `on_verify` with the PIN); the
//! component renders the prompt / locked label and requests focus.
//!
//! Apps that need richer behaviour (inline error display, status-banner
//! clearing, lockout polling) compose this with their own state.

use shared_core::i18n::Language;
use yew::prelude::*;

/// Props for [`Login`].
#[derive(Properties, PartialEq)]
pub struct LoginProps {
    /// Whether a PIN is required. When `false`, the component renders
    /// nothing and emits `on_login_success(false)` on mount.
    pub pin_required: bool,
    /// Length of the expected PIN (typically `4` or `8`).
    #[prop_or(4_usize)]
    pub pin_length: usize,
    /// Whether the backend reports the client is currently locked out.
    #[prop_or_default]
    pub locked: bool,
    /// Fires when the user finishes entering a PIN. The parent should
    /// call its verify-pin API and report success via `on_login_success`.
    pub on_verify: Callback<String>,
    /// Fires when login completes. `true` for verified, `false` for
    /// skipped (no PIN configured).
    pub on_login_success: Callback<bool>,
    /// Title shown when the user is not locked out.
    pub prompt_text: String,
    /// Title shown when the user is locked out.
    pub locked_text: String,
    /// Auto-focus the input on mount.
    #[prop_or(true)]
    pub autofocus: bool,
    /// Optional language for default text labels when props aren't set.
    pub language: Option<Language>,
    /// Optional element id for the form, useful for tests.
    #[prop_or_default]
    pub form_id: Option<String>,
}

/// Numeric-PIN login form.
#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let pin_input = use_state(String::new);
    let input_ref = use_node_ref();
    let locked = props.locked;
    let pin_len = props.pin_length;

    // Auto-emit `on_login_success(false)` when no PIN is required.
    {
        let on_success = props.on_login_success.clone();
        let pin_required = props.pin_required;
        use_effect_with((), move |_| {
            if !pin_required {
                on_success.emit(false);
            }
            || ()
        });
    }

    // Auto-focus the input on (re-)render when not locked.
    {
        let input_ref = input_ref.clone();
        let autofocus = props.autofocus;
        use_effect_with(locked, move |locked| {
            if !*locked
                && autofocus
                && let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>()
            {
                let _ = input.focus();
            }
            || ()
        });
    }

    let on_input = {
        let pin_input = pin_input.clone();
        let on_verify = props.on_verify.clone();
        let pin_len = props.pin_length;
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            let filtered: String = val.chars().filter(|c| c.is_ascii_digit()).collect();
            input.set_value(&filtered);

            if filtered.len() <= pin_len {
                pin_input.set(filtered.clone());
                if filtered.len() == pin_len {
                    on_verify.emit(filtered);
                }
            }
        })
    };

    let on_submit = {
        let pin_input = pin_input.clone();
        let on_verify = props.on_verify.clone();
        let pin_len = props.pin_length;
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let val = (*pin_input).clone();
            if val.len() == pin_len {
                on_verify.emit(val);
            }
        })
    };

    let placeholder = "•".repeat(pin_len);
    let form_id = props
        .form_id
        .clone()
        .unwrap_or_else(|| "pin-form".to_string());

    html! {
        <div class="login-container">
            <div class="login-box">
                <div class="pin-header">
                    <h2 id="pin-description">
                        { if locked { &props.locked_text } else { &props.prompt_text } }
                    </h2>
                </div>
                <form id={form_id} onsubmit={on_submit}>
                    <div class="pin-wrapper">
                        <input
                            ref={input_ref.clone()}
                            type="password"
                            class="pin-input-field"
                            value={(*pin_input).clone()}
                            oninput={on_input}
                            disabled={locked}
                            placeholder={placeholder}
                            maxlength={pin_len.to_string()}
                            autofocus={props.autofocus}
                        />
                    </div>
                </form>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pin_length_is_four() {
        let p = LoginProps {
            pin_required: true,
            pin_length: 4,
            locked: false,
            on_verify: Callback::noop(),
            on_login_success: Callback::noop(),
            prompt_text: "Enter PIN".to_string(),
            locked_text: "Locked".to_string(),
            autofocus: true,
            language: Some(Language::English),
            form_id: None,
        };
        assert_eq!(p.pin_length, 4);
        assert!(p.autofocus);
    }
}

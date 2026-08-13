//! Shared header component — title bar with theme/language/print/logout controls.

mod controls;

use crate::i18n::Language;
use crate::i18n::strings::StringKey;
use crate::theme::Theme;
use yew::prelude::*;

use controls::{
    language_picker, logout_button, logout_button_disabled, print_button, print_button_disabled,
    theme_toggle, tooltip_or_override,
};

/// Props for [`Header`].
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub site_title: String,
    /// Theme name (e.g. `"crateria"`, `"brinstar"`). Parsed into the
    /// [`Theme`] enum inside the component; unrecognised names fall
    /// back to [`Theme::default`].
    #[prop_or_default]
    pub theme: String,
    pub language: Language,
    pub toggle_theme: Callback<MouseEvent>,
    pub on_language_change: Callback<Language>,
    pub is_authenticated: bool,
    pub pin_required: bool,
    pub on_logout: Callback<MouseEvent>,

    #[prop_or_default]
    pub logout_tooltip: String,
    #[prop_or_default]
    pub theme_toggle_tooltip: String,
    #[prop_or_default]
    pub print_tooltip: String,
    pub on_print: Option<Callback<MouseEvent>>,

    #[prop_or(true)]
    pub enable_translation: bool,
    #[prop_or(true)]
    pub enable_themes: bool,
    #[prop_or(true)]
    pub enable_print: bool,
    pub print_disabled: bool,

    /// Explicit URL for the title link. Wins over [`Self::repo`].
    /// When both this and `repo` are `None`, a GitHub URL is derived from
    /// `site_title` (see [`resolve_title_href`]).
    #[prop_or_default]
    pub site_url: Option<String>,
    /// studio2201 repo slug for the title link, e.g. `"probe"` →
    /// `https://github.com/studio2201/probe`. Preferred over deriving from
    /// the display title so a custom `SITE_TITLE` still links to the repo.
    #[prop_or_default]
    pub repo: Option<String>,
    #[prop_or_default]
    pub version: Option<String>,
    #[prop_or_default]
    pub version_url: Option<String>,
}

/// Build the header title `href`.
///
/// Priority:
/// 1. explicit `site_url` if non-empty
/// 2. `https://github.com/studio2201/{repo}` if `repo` is non-empty
/// 3. `https://github.com/studio2201/{slug}` where `slug` is `site_title`
///    lowercased with non-alphanumerics stripped
#[must_use]
pub fn resolve_title_href(
    site_url: Option<&str>,
    repo: Option<&str>,
    site_title: &str,
) -> String {
    if let Some(url) = site_url.map(str::trim).filter(|s| !s.is_empty()) {
        return url.to_string();
    }
    if let Some(repo) = repo.map(str::trim).filter(|s| !s.is_empty()) {
        return format!("https://github.com/studio2201/{repo}");
    }
    let slug: String = site_title
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    format!("https://github.com/studio2201/{slug}")
}

/// Top-of-page navigation bar shared by all companion apps.
#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_change_lang = {
        let on_lang_change = props.on_language_change.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            on_lang_change.emit(Language::from_code(&select.value()));
        })
    };

    let logout_disabled = logout_button_disabled(props.is_authenticated, props.pin_required);
    let onclick_logout = if logout_disabled {
        Callback::from(|_| ())
    } else {
        props.on_logout.clone()
    };

    let theme_tooltip = tooltip_or_override(
        &props.theme_toggle_tooltip,
        StringKey::TooltipToggleTheme,
        props.language,
    );
    let print_tooltip = tooltip_or_override(
        &props.print_tooltip,
        StringKey::TooltipPrint,
        props.language,
    );
    let logout_tooltip = tooltip_or_override(
        &props.logout_tooltip,
        StringKey::TooltipLogout,
        props.language,
    );

    let print_allowed = !props.pin_required || props.is_authenticated;
    let on_print_prop = props.on_print.clone();
    let on_print = Callback::from(move |e: MouseEvent| {
        if !print_allowed {
            return;
        }
        if let Some(ref cb) = on_print_prop {
            cb.emit(e);
        } else if let Some(window) = web_sys::window() {
            let _ = window.print();
        }
    });

    let print_disabled = print_button_disabled(
        props.pin_required,
        props.is_authenticated,
        props.print_disabled,
    );

    // Parse the theme name into the `Theme` enum. Accept either the
    // kebab-case CSS names ("wrecked_ship") or any other value the
    // user stored in localStorage; unknown values fall back to default.
    let theme = Theme::from_name(&props.theme).unwrap_or_default();

    // Register global keyboard listener to cycle themes on "t" keypress.
    {
        let toggle_theme = props.toggle_theme.clone();
        let enable_themes = props.enable_themes;
        use_effect_with((), move |_| {
            if !enable_themes {
                return Box::new(|| ()) as Box<dyn FnOnce()>;
            }
            use wasm_bindgen::JsCast;
            let window = web_sys::window().unwrap();
            let toggle_theme = toggle_theme.clone();
            let listener =
                crate::utils::EventListener::new(&window, "keydown", move |e: web_sys::Event| {
                    let key_event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                    let key = key_event.key();

                    // Skip if focus is on form input/textarea/select so we don't disrupt typing.
                    if let Some(target) = e.target()
                        && let Ok(elem) = target.dyn_into::<web_sys::Element>()
                    {
                        let tag_name = elem.tag_name().to_lowercase();
                        if tag_name == "input" || tag_name == "textarea" || tag_name == "select" {
                            return;
                        }
                    }

                    if key == "t" || key == "T" {
                        // Create a mock MouseEvent to invoke the callback
                        if let Ok(dummy_event) = web_sys::MouseEvent::new("click") {
                            toggle_theme.emit(dummy_event);
                        }
                    }
                });
            Box::new(move || drop(listener)) as Box<dyn FnOnce()>
        });
    }

    let title_href = resolve_title_href(
        props.site_url.as_deref(),
        props.repo.as_deref(),
        &props.site_title,
    );
    let title_label = format!("Open {} on GitHub", props.site_title);

    let title_html = html! {
        <a
            class="header-title-link"
            href={title_href}
            target="_blank"
            rel="noopener noreferrer"
            title={title_label.clone()}
            aria-label={title_label}
        >
            <h1>{&props.site_title}</h1>
        </a>
    };

    html! {
        <header>
            <div id="header-title">
                {title_html}
            </div>

            <div class="header-right">
                {language_picker(props.enable_translation, props.language, on_change_lang)}
                {theme_toggle(props.enable_themes, props.toggle_theme.clone(), theme, theme_tooltip)}
                {print_button(props.enable_print, print_disabled, on_print, print_tooltip)}
                {logout_button(props.pin_required, logout_disabled, onclick_logout, logout_tooltip)}
            </div>
        </header>
    }
}

#[cfg(test)]
mod tests;

//! Shared footer component — version, optional children, optional GitHub link.

use crate::i18n::Language;
use crate::i18n::strings::{StringKey, lookup};
use yew::prelude::*;

/// Props for [`Footer`].
#[derive(Properties, PartialEq, Clone)]
pub struct FooterProps {
    #[prop_or_default]
    pub show_version: bool,
    #[prop_or_default]
    pub version: String,
    #[prop_or(true)]
    pub show_github: bool,
    #[prop_or_default]
    pub github_url: Option<String>,

    #[prop_or_default]
    pub version_url: Option<String>,

    #[prop_or(true)]
    pub show_coffee: bool,
    #[prop_or_default]
    pub coffee_url: Option<String>,

    #[prop_or_default]
    pub children: Html,
}

/// Bottom-of-page footer shared by all companion apps.
#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let github_link = props
        .github_url
        .clone()
        .unwrap_or_else(|| "https://github.com/studio2201".to_string());

    let coffee_link = props
        .coffee_url
        .clone()
        .unwrap_or_else(|| "https://www.buymeacoffee.com/ubermetroid".to_string());

    let aria_github = lookup(StringKey::AriaGitHubProfile, Language::English);

    html! {
        <footer class="layout-footer">
            <div class="footer-left">
                {version_block(props.show_version, &props.version, props.version_url.as_deref())}
            </div>

            <div class="footer-center">
                {props.children.clone()}
            </div>

            <div class="footer-right">
            </div>
        </footer>
    }
}

/// Renders the version link or static version text, depending on whether a URL is set.
fn version_block(show: bool, version: &str, url: Option<&str>) -> Html {
    if !show {
        return html! {};
    }
    let display = format!("v{version}");
    match url {
        Some(u) => {
            let title = lookup(StringKey::TitleViewReleaseNotes, Language::English);
            html! {
                <a class="footer-version-link"
                   href={u.to_string()}
                   target="_blank"
                   rel="noopener noreferrer"
                   title={title}>
                    {display}
                </a>
            }
        }
        None => html! {
            <span class="footer-version">{display}</span>
        },
    }
}

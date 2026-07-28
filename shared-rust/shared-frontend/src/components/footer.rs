//! Shared footer component — version, optional children, optional GitHub link.

use crate::i18n::Language;
use crate::i18n::strings::{StringKey, lookup};
use yew::prelude::*;

/// Props for [`Footer`].
#[derive(Properties, PartialEq, Clone)]
pub struct FooterProps {
    #[prop_or_default]
    pub show_version: bool,
    /// Package version string, with or without a leading `v` (e.g. `"0.1.1"`).
    #[prop_or_default]
    pub version: String,
    #[prop_or(true)]
    pub show_github: bool,
    #[prop_or_default]
    pub github_url: Option<String>,

    /// Explicit release notes URL. Wins over the URL derived from [`Self::repo`].
    #[prop_or_default]
    pub version_url: Option<String>,

    /// studio2201 repo slug (e.g. `"probe"`). Used to build the GitHub release
    /// tag link when `version_url` is unset:
    /// `https://github.com/studio2201/{repo}/releases/tag/v{version}`.
    #[prop_or_default]
    pub repo: Option<String>,

    #[prop_or(true)]
    pub show_coffee: bool,
    #[prop_or_default]
    pub coffee_url: Option<String>,

    #[prop_or_default]
    pub children: Html,
}

impl Default for FooterProps {
    fn default() -> Self {
        Self {
            show_version: false,
            version: String::new(),
            show_github: true,
            github_url: None,
            version_url: None,
            repo: None,
            show_coffee: true,
            coffee_url: None,
            children: Html::default(),
        }
    }
}

/// Normalize a version for display / tag URLs: strip one leading `v`/`V`.
#[must_use]
pub fn normalize_version_tag(version: &str) -> String {
    let v = version.trim();
    let stripped = v
        .strip_prefix('v')
        .or_else(|| v.strip_prefix('V'))
        .unwrap_or(v);
    stripped.to_string()
}

/// Build the footer version `href` (GitHub release tag when possible).
///
/// Priority:
/// 1. explicit non-empty `version_url`
/// 2. `https://github.com/studio2201/{repo}/releases/tag/v{version}` when
///    both `repo` and `version` are non-empty
/// 3. `None` (render as plain text)
#[must_use]
pub fn resolve_version_href(
    version_url: Option<&str>,
    repo: Option<&str>,
    version: &str,
) -> Option<String> {
    if let Some(url) = version_url.map(str::trim).filter(|s| !s.is_empty()) {
        return Some(url.to_string());
    }
    let repo = repo.map(str::trim).filter(|s| !s.is_empty())?;
    let tag = normalize_version_tag(version);
    if tag.is_empty() {
        return None;
    }
    Some(format!(
        "https://github.com/studio2201/{repo}/releases/tag/v{tag}"
    ))
}

/// Bottom-of-page footer shared by all companion apps.
#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let _github_url = props
        .github_url
        .clone()
        .unwrap_or_else(|| "https://github.com/studio2201".to_string());

    let _coffee_url = props
        .coffee_url
        .clone()
        .unwrap_or_else(|| "https://www.buymeacoffee.com/ubermetroid".to_string());

    let _aria_github = lookup(StringKey::AriaGitHubProfile, Language::English);

    let version_href = resolve_version_href(
        props.version_url.as_deref(),
        props.repo.as_deref(),
        &props.version,
    );

    html! {
        <footer class="layout-footer">
            <div class="footer-left">
                {version_block(props.show_version, &props.version, version_href.as_deref())}
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
    if !show || version.trim().is_empty() {
        return html! {};
    }
    let tag = normalize_version_tag(version);
    let display = format!("v{tag}");
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

#[cfg(test)]
mod tests {
    use super::{normalize_version_tag, resolve_version_href};

    #[test]
    fn normalize_strips_leading_v() {
        assert_eq!(normalize_version_tag("0.1.1"), "0.1.1");
        assert_eq!(normalize_version_tag("v0.1.1"), "0.1.1");
        assert_eq!(normalize_version_tag("V1.0.28"), "1.0.28");
        assert_eq!(normalize_version_tag("  v2.0.0  "), "2.0.0");
    }

    #[test]
    fn explicit_version_url_wins() {
        assert_eq!(
            resolve_version_href(Some("https://example.com/notes"), Some("probe"), "0.1.1")
                .as_deref(),
            Some("https://example.com/notes")
        );
    }

    #[test]
    fn repo_and_version_build_release_tag_url() {
        assert_eq!(
            resolve_version_href(None, Some("probe"), "0.1.1").as_deref(),
            Some("https://github.com/studio2201/probe/releases/tag/v0.1.1")
        );
        // avoid vv when version already has v
        assert_eq!(
            resolve_version_href(None, Some("mark"), "v0.1.1").as_deref(),
            Some("https://github.com/studio2201/mark/releases/tag/v0.1.1")
        );
    }

    #[test]
    fn missing_repo_or_version_yields_none() {
        assert!(resolve_version_href(None, None, "0.1.1").is_none());
        assert!(resolve_version_href(None, Some("probe"), "").is_none());
        assert!(resolve_version_href(None, Some("  "), "0.1.1").is_none());
    }
}

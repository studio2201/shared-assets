use super::resolve_title_href;

#[test]
fn explicit_site_url_wins() {
    assert_eq!(
        resolve_title_href(Some("https://example.com/x"), Some("probe"), "Probe"),
        "https://example.com/x"
    );
}

#[test]
fn repo_slug_builds_github_url() {
    assert_eq!(
        resolve_title_href(None, Some("probe"), "Custom Title"),
        "https://github.com/studio2201/probe"
    );
}

#[test]
fn title_fallback_lowercases_and_strips() {
    assert_eq!(
        resolve_title_href(None, None, "Probe"),
        "https://github.com/studio2201/probe"
    );
    assert_eq!(
        resolve_title_href(None, None, "StateSync"),
        "https://github.com/studio2201/statesync"
    );
    // spaces / punctuation stripped so display titles still map to a repo slug
    assert_eq!(
        resolve_title_href(None, None, "My Probe!"),
        "https://github.com/studio2201/myprobe"
    );
}

#[test]
fn empty_site_url_falls_through_to_repo() {
    assert_eq!(
        resolve_title_href(Some("  "), Some("mark"), "Mark"),
        "https://github.com/studio2201/mark"
    );
}

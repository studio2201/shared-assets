//! Title injection middleware.
//!
//! Replaces `{{SITE_TITLE}}` placeholders in HTML responses served from the
//! configured static directory. Apps can extend the placeholder set with
//! app-specific values by wrapping the inner service.

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use http_body_util::BodyExt;
use std::sync::Arc;

use crate::server::ServerConfig;

/// State for the title-injection middleware. Wrap your `ServerConfig` in an
/// `Arc` and hand it to [`title_injection_layer`].
#[derive(Clone)]
pub struct TitleState(pub Arc<ServerConfig>);

/// Middleware that replaces `{{SITE_TITLE}}` in text/html responses.
///
/// **Caveats**:
///
/// - Buffers the entire response body in memory before replacement.
///   Fine for small `index.html` files; not suitable for streaming HTML.
/// - Does not update `Content-Length` after replacement. For chunked
///   transfer this is irrelevant; for fixed-length responses the
///   client may see a stale length.
pub async fn title_injection_layer(
    State(state): State<TitleState>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

    let is_html = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|s| s.starts_with("text/html"));

    if !is_html {
        return response;
    }

    let (parts, body) = response.into_parts();
    let bytes = match body.collect().await {
        Ok(c) => c.to_bytes(),
        Err(_) => {
            return Response::from_parts(parts, Body::empty());
        }
    };

    let body_str = match std::str::from_utf8(&bytes) {
        Ok(s) => s,
        Err(_) => {
            return Response::from_parts(parts, Body::from(bytes));
        }
    };

    // Escape so a compromised/hostile SITE_TITLE cannot inject HTML/JS into
    // every page that uses {{SITE_TITLE}} (shared across companion apps).
    let safe_title = html_escape(&state.0.site_title);
    let injected = body_str.replace("{{SITE_TITLE}}", &safe_title);
    Response::from_parts(parts, Body::from(injected))
}

/// Minimal HTML entity escape for text content / attribute-safe titles.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles() {
        fn _exists() {
            let _: fn(
                axum::extract::State<TitleState>,
                axum::extract::Request,
                axum::middleware::Next,
            ) -> _ = title_injection_layer;
        }
    }

    #[test]
    fn html_escape_blocks_script_payload() {
        let raw = r#"Todo</title><script>alert(1)</script>"#;
        let esc = html_escape(raw);
        assert!(!esc.contains('<'));
        assert!(!esc.contains('>'));
        assert!(esc.contains("&lt;script&gt;"));
    }
}

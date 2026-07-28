//! Shared notifier / toast components for every companion app.

use yew::prelude::*;

/// Visual style of a toast notification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToastType {
    #[default]
    Info,
    Success,
    Error,
    Warning,
}

impl ToastType {
    /// CSS class suffix applied as `toast.{class}`.
    #[must_use]
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Info => "info",
            Self::Warning => "warning",
        }
    }
}

/// Props for a single toast.
#[derive(Properties, PartialEq)]
pub struct ToastNotificationProps {
    pub message: String,
    #[prop_or_default]
    pub toast_type: ToastType,
    #[prop_or_default]
    pub on_dismiss: Callback<()>,
}

/// One floating toast pill.
#[function_component(ToastNotification)]
pub fn toast_notification(props: &ToastNotificationProps) -> Html {
    let onclick = {
        let on_dismiss = props.on_dismiss.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_dismiss.emit(());
        })
    };
    html! {
        <div
            class={format!("toast show {}", props.toast_type.as_class())}
            role="status"
            onclick={onclick}
        >
            { &props.message }
        </div>
    }
}

/// Props for the toast stack container.
#[derive(Properties, PartialEq)]
pub struct ToastContainerProps {
    pub children: Children,
}

/// Fixed-position stack for toast children.
#[function_component(ToastContainer)]
pub fn toast_container(props: &ToastContainerProps) -> Html {
    html! {
        <div class="toast-container" aria-live="polite">
            { for props.children.iter() }
        </div>
    }
}

/// Props for a simple inline banner (not a floating toast).
#[derive(Properties, PartialEq)]
pub struct BannerProps {
    pub message: String,
    #[prop_or(ToastType::Info)]
    pub kind: ToastType,
    #[prop_or_default]
    pub class: Classes,
}

/// Inline status banner for page-level messages.
#[function_component(Banner)]
pub fn banner(props: &BannerProps) -> Html {
    let kind_class = match props.kind {
        ToastType::Success => "banner-success",
        ToastType::Error => "banner-error",
        ToastType::Info | ToastType::Warning => "banner-info",
    };
    html! {
        <div class={classes!("banner", kind_class, props.class.clone())} role="status">
            { &props.message }
        </div>
    }
}

//! Full-page chrome shell: header slot + main + footer + optional toasts.
//!
//! Apps pass product content as `children` (main). Header/footer use the
//! shared components so every companion app shares one layout structure.

use yew::prelude::*;

use super::footer::{Footer, FooterProps};
use super::header::{Header, HeaderProps};
use super::notifier::ToastContainer;

/// Props for [`AppShell`].
#[derive(Properties, PartialEq)]
pub struct AppShellProps {
    /// Shared header configuration.
    pub header: HeaderProps,
    /// Shared footer configuration.
    #[prop_or_default]
    pub footer: FooterProps,
    /// Main product content.
    pub children: Children,
    /// Optional toast stack (already rendered `ToastNotification` nodes).
    /// When `None`, no toast container is rendered.
    #[prop_or_default]
    pub toasts: Option<Html>,
    /// Extra class on the outer shell (`app-shell` always applied).
    #[prop_or_default]
    pub class: Classes,
    /// Extra class on the main region.
    #[prop_or_default]
    pub main_class: Classes,
    /// When true, wrap main content in `.container`.
    #[prop_or(true)]
    pub use_container: bool,
}

/// Standard studio2201 page frame.
#[function_component(AppShell)]
pub fn app_shell(props: &AppShellProps) -> Html {
    let shell_class = classes!("app-shell", props.class.clone());
    let main_class = classes!("app-shell-main", props.main_class.clone());

    let main_inner = if props.use_container {
        html! {
            <div class="container">
                { for props.children.iter() }
            </div>
        }
    } else {
        html! { <>{ for props.children.iter() }</> }
    };

    html! {
        <div class={shell_class}>
            <Header
                site_title={props.header.site_title.clone()}
                theme={props.header.theme.clone()}
                language={props.header.language}
                toggle_theme={props.header.toggle_theme.clone()}
                on_language_change={props.header.on_language_change.clone()}
                is_authenticated={props.header.is_authenticated}
                pin_required={props.header.pin_required}
                on_logout={props.header.on_logout.clone()}
                logout_tooltip={props.header.logout_tooltip.clone()}
                theme_toggle_tooltip={props.header.theme_toggle_tooltip.clone()}
                print_tooltip={props.header.print_tooltip.clone()}
                on_print={props.header.on_print.clone()}
                enable_translation={props.header.enable_translation}
                enable_themes={props.header.enable_themes}
                enable_print={props.header.enable_print}
                print_disabled={props.header.print_disabled}
                site_url={props.header.site_url.clone()}
                version={props.header.version.clone()}
                version_url={props.header.version_url.clone()}
            />
            <main class={main_class}>
                { main_inner }
            </main>
            <Footer
                show_version={props.footer.show_version}
                version={props.footer.version.clone()}
                show_github={props.footer.show_github}
                github_url={props.footer.github_url.clone()}
                version_url={props.footer.version_url.clone()}
                show_coffee={props.footer.show_coffee}
                coffee_url={props.footer.coffee_url.clone()}
            >
                { props.footer.children.clone() }
            </Footer>
            if let Some(toasts) = props.toasts.clone() {
                <ToastContainer>
                    { toasts }
                </ToastContainer>
            }
        </div>
    }
}

pub mod components;
pub mod locale;
pub mod storage;
pub mod theme;
pub mod utils;
pub use locale::{detect_browser_locale, get_saved_locale, set_saved_locale};
pub use storage::StorageService;
pub use utils::EventListener;

pub mod i18n;

// Re-exports — web UI kit surface for every companion app
pub use components::{
    app_shell,
    app_shell::{AppShell, AppShellProps},
    footer,
    footer::{Footer, FooterProps},
    header,
    header::{Header, HeaderProps},
    language_switcher,
    language_switcher::{LanguageSwitcher, LanguageSwitcherProps},
    login,
    login::{Login, LoginProps, filter_numeric_pin},
    notifier,
    notifier::{
        Banner, BannerProps, ToastContainer, ToastContainerProps, ToastNotification,
        ToastNotificationProps, ToastType,
    },
};

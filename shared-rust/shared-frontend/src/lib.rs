pub mod components;
pub mod locale;
pub mod storage;
pub mod theme;
pub mod utils;
pub use locale::{detect_browser_locale, get_saved_locale, set_saved_locale};
pub use storage::StorageService;
pub use utils::EventListener;

pub mod i18n;

// Re-exports for ergonomics
pub use components::{
    footer,
    footer::Footer,
    header,
    header::Header,
    language_switcher,
    language_switcher::{LanguageSwitcher, LanguageSwitcherProps},
    login,
    login::{Login, LoginProps},
    notifier,
    notifier::{ToastContainer, ToastNotification, ToastType},
};

//! Frontend-specific i18n: a base set of common UI strings.
//!
//! Re-exports the [`Language`] enum and [`strings`] module from
//! `shared_core::i18n` so existing components keep resolving
//! `crate::i18n::Language` and `crate::i18n::strings::StringKey`, and adds
//! [`common_strings`] for the base UI vocabulary every app can use.

pub mod common_strings;
pub use common_strings::{CommonString, lookup};

pub use shared_core::i18n::*;

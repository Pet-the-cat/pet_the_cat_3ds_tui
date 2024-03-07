/// Application.
pub mod app;

/// Screen renderer.
pub mod ui;

/// Event handler.
pub mod handler;

/// Localization manager.
pub mod localization;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!();
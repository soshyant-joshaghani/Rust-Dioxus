//! Light / dark theme. Dark by default, persisted under `theme` like Rust-Svelte.

use dioxus::prelude::*;

use super::storage;

const STORAGE_KEY: &str = "theme";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn is_dark(self) -> bool {
        self == Theme::Dark
    }
}

#[derive(Clone, Copy)]
pub struct ThemeStore(pub Signal<Theme>);

impl ThemeStore {
    pub fn get(&self) -> Theme {
        *self.0.read()
    }

    pub fn toggle(mut self) {
        let next = if self.get().is_dark() { Theme::Light } else { Theme::Dark };
        storage::set(STORAGE_KEY, if next.is_dark() { "dark" } else { "light" });
        self.0.set(next);
    }
}

pub fn use_theme() -> ThemeStore {
    use_context::<ThemeStore>()
}

pub fn use_theme_provider() -> ThemeStore {
    use_context_provider(|| {
        let initial = match storage::get(STORAGE_KEY).as_deref() {
            Some("light") => Theme::Light,
            _ => Theme::Dark,
        };
        ThemeStore(Signal::new(initial))
    })
}

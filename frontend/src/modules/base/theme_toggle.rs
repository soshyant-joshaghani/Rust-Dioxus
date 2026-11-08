use dioxus::prelude::*;

use crate::modules::base::icons::{MoonIcon, SunIcon};
use crate::modules::base::stores::theme::use_theme;
use crate::modules::base::ui::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ThemeToggle() -> Element {
    let theme = use_theme();
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            class: "relative",
            aria_label: "Toggle theme",
            onclick: move |_| theme.toggle(),
            SunIcon { class: "size-4 scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90" }
            MoonIcon { class: "absolute size-4 scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0" }
        }
    }
}

use dioxus::prelude::*;

/// Initials avatar (the shell has no profile images).
#[component]
pub fn Avatar(#[props(into)] fallback: String, #[props(into, default)] class: String) -> Element {
    rsx! {
        span { class: "relative flex size-8 shrink-0 overflow-hidden rounded-full {class}",
            span { class: "flex size-full items-center justify-center rounded-full bg-muted text-xs font-medium",
                "{fallback}"
            }
        }
    }
}

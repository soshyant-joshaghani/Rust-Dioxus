use dioxus::prelude::*;

use crate::modules::base::icons::XIcon;

/// Right-side panel over a dimmed overlay. Clicking the overlay or the close button calls `onclose`.
#[component]
pub fn Sheet(
    open: bool,
    #[props(into)] title: String,
    #[props(into, default)] description: String,
    onclose: EventHandler<()>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }
    rsx! {
        div { class: "fixed inset-0 z-50 bg-black/50", onclick: move |_| onclose.call(()) }
        div {
            role: "dialog",
            class: "fixed inset-y-0 right-0 z-50 flex h-full w-3/4 flex-col gap-4 border-l bg-background shadow-lg sm:max-w-md",
            div { class: "flex flex-col gap-1.5 p-4",
                h2 { class: "font-semibold text-foreground", "{title}" }
                if !description.is_empty() {
                    p { class: "text-sm text-muted-foreground", "{description}" }
                }
            }
            button {
                r#type: "button",
                class: "absolute top-4 right-4 cursor-pointer rounded-xs opacity-70 transition-opacity hover:opacity-100",
                "aria-label": "Close",
                onclick: move |_| onclose.call(()),
                XIcon {}
            }
            {children}
        }
    }
}

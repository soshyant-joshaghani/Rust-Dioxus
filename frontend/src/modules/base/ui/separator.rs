use dioxus::prelude::*;

#[component]
pub fn Separator(#[props(default)] vertical: bool, #[props(into, default)] class: String) -> Element {
    let orientation = if vertical { "h-full w-px" } else { "h-px w-full" };
    rsx! {
        div { role: "separator", class: "shrink-0 bg-border {orientation} {class}" }
    }
}

use dioxus::prelude::*;

#[component]
pub fn Switch(#[props(into, default)] id: String, checked: bool, onchange: EventHandler<bool>) -> Element {
    let track = if checked { "bg-primary" } else { "bg-input dark:bg-input/80" };
    let thumb = if checked { "translate-x-[calc(100%-2px)]" } else { "translate-x-0" };
    rsx! {
        button {
            id: "{id}",
            r#type: "button",
            role: "switch",
            "aria-checked": if checked { "true" } else { "false" },
            class: "inline-flex h-[1.15rem] w-8 shrink-0 cursor-pointer items-center rounded-full border border-transparent shadow-xs transition-all outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 {track}",
            onclick: move |_| onchange.call(!checked),
            span { class: "pointer-events-none block size-4 rounded-full bg-background ring-0 transition-transform dark:bg-foreground {thumb}" }
        }
    }
}

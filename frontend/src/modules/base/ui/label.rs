use dioxus::prelude::*;

#[component]
pub fn Label(#[props(into, default)] r#for: String, #[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        label {
            r#for: "{r#for}",
            class: "flex items-center gap-2 text-sm leading-none font-medium select-none {class}",
            {children}
        }
    }
}

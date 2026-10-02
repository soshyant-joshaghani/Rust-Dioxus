use dioxus::prelude::*;

#[component]
pub fn Card(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 rounded-xl border bg-card py-6 text-card-foreground shadow-sm {class}", {children} }
    }
}

#[component]
pub fn CardHeader(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "grid auto-rows-min items-start gap-1.5 px-6 {class}", {children} }
    }
}

#[component]
pub fn CardTitle(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "leading-none font-semibold {class}", {children} }
    }
}

#[component]
pub fn CardDescription(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "text-sm text-muted-foreground {class}", {children} }
    }
}

#[component]
pub fn CardContent(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "px-6 {class}", {children} }
    }
}

#[component]
pub fn CardFooter(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "flex items-center px-6 {class}", {children} }
    }
}

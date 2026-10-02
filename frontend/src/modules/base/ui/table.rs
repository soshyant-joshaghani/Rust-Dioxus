use dioxus::prelude::*;

#[component]
pub fn Table(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "relative w-full overflow-x-auto",
            table { class: "w-full caption-bottom text-sm {class}", {children} }
        }
    }
}

#[component]
pub fn TableHeader(children: Element) -> Element {
    rsx! { thead { class: "[&_tr]:border-b", {children} } }
}

#[component]
pub fn TableBody(children: Element) -> Element {
    rsx! { tbody { class: "[&_tr:last-child]:border-0", {children} } }
}

#[component]
pub fn TableRow(
    #[props(into, default)] class: String,
    #[props(default)] selected: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let state = if selected { "bg-muted" } else { "" };
    rsx! {
        tr {
            class: "border-b transition-colors hover:bg-muted/50 {state} {class}",
            onclick: move |evt| {
                if let Some(handler) = onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn TableHead(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        th { class: "h-10 px-2 text-left align-middle font-medium whitespace-nowrap text-foreground {class}", {children} }
    }
}

#[component]
pub fn TableCell(#[props(into, default)] class: String, #[props(default = 1)] colspan: u32, children: Element) -> Element {
    rsx! {
        td { class: "p-2 align-middle whitespace-nowrap {class}", colspan: "{colspan}", {children} }
    }
}

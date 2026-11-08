//! Collapsible sidebar. Desktop widths collapse in place; small screens open it as an overlay.

use dioxus::prelude::*;

use crate::modules::base::icons::PanelLeftIcon;

#[derive(Clone, Copy)]
pub struct SidebarState {
    /// md and up: sidebar hidden when true.
    pub collapsed: Signal<bool>,
    /// below md: overlay shown when true.
    pub mobile_open: Signal<bool>,
}

pub fn use_sidebar_provider() -> SidebarState {
    use_context_provider(|| SidebarState { collapsed: Signal::new(false), mobile_open: Signal::new(false) })
}

pub fn use_sidebar() -> SidebarState {
    use_context::<SidebarState>()
}

/// The sidebar shell. Content (header, menu, footer) is passed as children.
#[component]
pub fn Sidebar(children: Element) -> Element {
    let state = use_sidebar();
    let mut mobile_open = state.mobile_open;
    let desktop = if (state.collapsed)() { "md:hidden" } else { "md:flex" };
    let mobile = if mobile_open() { "flex" } else { "hidden" };
    rsx! {
        if mobile_open() {
            div { class: "fixed inset-0 z-40 bg-black/50 md:hidden", onclick: move |_| mobile_open.set(false) }
        }
        aside {
            class: "fixed inset-y-0 left-0 z-50 w-64 shrink-0 flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground md:sticky md:top-0 md:z-auto md:h-screen {mobile} {desktop}",
            {children}
        }
    }
}

#[component]
pub fn SidebarMenuButton(active: bool, children: Element) -> Element {
    let state = if active {
        "bg-sidebar-accent font-medium text-sidebar-accent-foreground"
    } else {
        "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
    };
    rsx! {
        div { class: "flex w-full items-center gap-2 rounded-md p-2 text-left text-sm [&>svg]:size-4 {state}", {children} }
    }
}

/// Toggles the sidebar: collapses it on desktop widths, opens the overlay on small screens.
#[component]
pub fn SidebarTrigger(#[props(into, default)] class: String) -> Element {
    let state = use_sidebar();
    let mut collapsed = state.collapsed;
    let mut mobile_open = state.mobile_open;
    rsx! {
        button {
            r#type: "button",
            class: "inline-flex size-7 cursor-pointer items-center justify-center rounded-md hover:bg-accent hover:text-accent-foreground md:hidden {class}",
            "aria-label": "Toggle sidebar",
            onclick: move |_| mobile_open.toggle(),
            PanelLeftIcon {}
        }
        button {
            r#type: "button",
            class: "hidden size-7 cursor-pointer items-center justify-center rounded-md hover:bg-accent hover:text-accent-foreground md:inline-flex {class}",
            "aria-label": "Toggle sidebar",
            onclick: move |_| collapsed.toggle(),
            PanelLeftIcon {}
        }
    }
}

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    fn classes(self) -> &'static str {
        match self {
            Self::Default => "border-transparent bg-primary text-primary-foreground",
            Self::Secondary => "border-transparent bg-secondary text-secondary-foreground",
            Self::Destructive => "border-transparent bg-destructive text-white",
            Self::Outline => "text-foreground",
        }
    }
}

#[component]
pub fn Badge(#[props(default)] variant: BadgeVariant, #[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        span {
            class: "inline-flex w-fit shrink-0 items-center justify-center gap-1 rounded-md border px-2 py-0.5 text-xs font-medium whitespace-nowrap {variant.classes()} {class}",
            {children}
        }
    }
}

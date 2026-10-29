use dioxus::prelude::*;

const BASE: &str = "flex h-9 w-full min-w-0 rounded-md border border-input bg-background px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30";

/// Controlled text input: pass the current `value` and update it in `oninput`.
#[component]
pub fn Input(
    #[props(into, default)] id: String,
    #[props(into, default = "text".to_string())] r#type: String,
    #[props(into)] value: String,
    #[props(into, default)] placeholder: String,
    #[props(into, default)] autocomplete: String,
    #[props(into, default)] class: String,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        input {
            id: "{id}",
            class: "{BASE} {class}",
            r#type: "{r#type}",
            value: "{value}",
            placeholder: "{placeholder}",
            autocomplete: "{autocomplete}",
            required,
            disabled,
            oninput: move |evt| oninput.call(evt.value()),
        }
    }
}

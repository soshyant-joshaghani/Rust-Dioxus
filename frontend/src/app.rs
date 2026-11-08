use dioxus::prelude::*;

use crate::modules::base::pwa::PwaRegister;
use crate::modules::base::stores::{auth::use_auth_provider, theme::use_theme_provider};
use crate::modules::base::APP_NAME;
use crate::routes::Route;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

/// Root: stylesheet, stores, theme, router. Shared by every platform.
#[component]
pub fn App() -> Element {
    use_auth_provider();
    let theme = use_theme_provider();
    let dark = if theme.get().is_dark() { "dark" } else { "" };

    rsx! {
        document::Title { "{APP_NAME}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        PwaRegister {}
        // The `dark` class on the wrapper switches every token (see tailwind.css).
        div { class: "{dark}",
            div { class: "min-h-screen bg-background font-sans text-foreground antialiased",
                Router::<Route> {}
            }
        }
    }
}

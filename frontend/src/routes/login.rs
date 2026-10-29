use dioxus::prelude::*;

use crate::modules::base::authentication::Authentication;
use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::APP_NAME;
use crate::routes::Route;

#[component]
pub fn Login() -> Element {
    let auth = use_auth();
    let nav = navigator();

    use_effect(move || {
        let state = auth.0.read();
        if !state.is_loading && state.is_authenticated() {
            nav.replace(Route::Home {});
        }
    });

    rsx! {
        div { class: "flex min-h-screen flex-col items-center justify-center bg-background p-6",
            div { class: "mb-8 text-center",
                p { class: "text-xs font-semibold tracking-widest text-muted-foreground uppercase", "Welcome to" }
                h1 { class: "text-3xl font-bold", "{APP_NAME}" }
            }
            div { class: "w-full max-w-md rounded-xl border border-border bg-card p-6 shadow-lg",
                Authentication {}
            }
        }
    }
}

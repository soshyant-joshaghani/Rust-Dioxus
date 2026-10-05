//! Client routes. Same paths as every FoxG client: `/`, `/login`, `/sample/notes`, `/admin`.
//! Add a page: create the file under `routes/`, then add a variant here.

use dioxus::prelude::*;

pub mod dashboard;
pub mod login;

use dashboard::{admin::Admin, home::Home, sample::notes::SampleNotes, DashboardLayout};
use login::Login;

#[derive(Debug, Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[route("/login")]
    Login {},

    #[layout(DashboardLayout)]
        #[route("/")]
        Home {},
        #[route("/sample/notes")]
        SampleNotes {},
        #[route("/admin")]
        Admin {},
    #[end_layout]

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = segments.join("/");
    rsx! {
        div { class: "flex min-h-screen flex-col items-center justify-center gap-4 p-6 text-center",
            h1 { class: "text-3xl font-bold", "Not found" }
            p { class: "text-muted-foreground", "/{path}" }
            Link { to: Route::Home {}, class: "text-primary underline-offset-4 hover:underline", "Back to the dashboard" }
        }
    }
}

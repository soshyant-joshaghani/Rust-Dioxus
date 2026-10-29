//! The authenticated area (Rust-Svelte's `routes/(dashboard)`): sidebar + header + page.

use dioxus::prelude::*;

pub mod admin;
pub mod home;
pub mod sample;

use crate::modules::base::app_sidebar::AppSidebar;
use crate::modules::base::header::Header;
use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::ui::sidebar::use_sidebar_provider;
use crate::routes::Route;

#[component]
pub fn DashboardLayout() -> Element {
    let auth = use_auth();
    let nav = navigator();
    use_sidebar_provider();

    use_effect(move || {
        let state = auth.0.read();
        if !state.is_loading && !state.is_authenticated() {
            nav.replace(Route::Login {});
        }
    });

    let state = auth.state();
    if state.is_loading {
        return rsx! {
            div { class: "flex min-h-screen items-center justify-center text-muted-foreground", "Restoring session…" }
        };
    }
    if !state.is_authenticated() {
        return rsx! {};
    }

    rsx! {
        div { class: "flex min-h-screen w-full",
            AppSidebar {}
            div { class: "flex min-w-0 flex-1 flex-col bg-background",
                Header {}
                main { class: "flex-1 p-6", Outlet::<Route> {} }
            }
        }
    }
}

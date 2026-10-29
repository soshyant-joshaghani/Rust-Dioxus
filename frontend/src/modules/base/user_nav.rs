use dioxus::prelude::*;

use crate::modules::base::icons::LogOutIcon;
use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::ui::{Avatar, Button, ButtonSize, ButtonVariant};
use crate::routes::Route;

#[component]
pub fn UserNav() -> Element {
    let auth = use_auth();
    let nav = navigator();
    let Some(user) = auth.state().user else {
        return rsx! {};
    };
    let initials: String = user.email.chars().take(2).collect::<String>().to_uppercase();
    let role = if user.is_superuser() { "SuperAdmin" } else { "User" };

    rsx! {
        div { class: "flex items-center gap-3",
            div { class: "hidden items-center gap-2 sm:flex",
                Avatar { fallback: initials }
                div { class: "text-right text-sm leading-tight",
                    p { class: "font-medium", "{user.email}" }
                    p { class: "text-xs text-muted-foreground", "{role}" }
                }
            }
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    auth.logout();
                    nav.replace(Route::Login {});
                },
                LogOutIcon {}
                "Log out"
            }
        }
    }
}

//! User management (superuser only), the Dioxus port of Rust-Svelte's admin page.

use dioxus::prelude::*;

use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::ui::{
    Button, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, Input, Label, Switch,
};
use crate::modules::base::users::api::{create_user, delete_user, list_users, update_user, ManagedUser, UserCreate, UserUpdate};
use crate::modules::base::utils::api_error::ApiError;
use crate::routes::Route;

#[component]
pub fn Admin() -> Element {
    let auth = use_auth();
    let nav = navigator();

    let mut users = use_signal(Vec::<ManagedUser>::new);
    let mut selected_id = use_signal(|| None::<String>);
    let mut email = use_signal(String::new);
    let mut full_name = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_active = use_signal(|| false);
    let mut is_superuser = use_signal(|| false);
    let mut status = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut saving = use_signal(|| false);

    use_effect(move || {
        let state = auth.0.read();
        if state.user.is_some() && !state.is_superuser() {
            nav.replace(Route::Home {});
        }
    });

    let mut fail = move |err: ApiError, fallback: &str| {
        if err.is_unauthorized() {
            auth.logout();
            nav.replace(Route::Login {});
        } else {
            status.set(if err.message.is_empty() { fallback.to_string() } else { err.message });
        }
    };

    let refresh = move || async move {
        let Some(token) = auth.token() else { return };
        loading.set(true);
        status.set(String::new());
        match list_users(&token).await {
            Ok(list) => {
                status.set(format!("{} user(s)", list.len()));
                users.set(list);
            }
            Err(err) => {
                users.set(Vec::new());
                fail(err, "Failed to load users");
            }
        }
        loading.set(false);
    };

    use_hook(move || {
        if auth.state().is_superuser() {
            spawn(refresh());
        }
    });

    let mut clear_form = move || {
        selected_id.set(None);
        email.set(String::new());
        full_name.set(String::new());
        password.set(String::new());
        is_active.set(false);
        is_superuser.set(false);
        status.set("New user".into());
    };

    let mut select_user = move |user: ManagedUser| {
        status.set(format!("Editing: {}", user.email));
        email.set(user.email);
        full_name.set(user.full_name.unwrap_or_default());
        password.set(String::new());
        is_active.set(user.is_active);
        is_superuser.set(user.is_superuser);
        selected_id.set(Some(user.id));
    };

    let on_save = move |evt: FormEvent| async move {
        evt.prevent_default();
        let Some(token) = auth.token() else { return };
        let trimmed_email = email().trim().to_string();
        if trimmed_email.is_empty() {
            status.set("Email is required".into());
            return;
        }
        let name = full_name().trim().to_string();
        let name = (!name.is_empty()).then_some(name);
        let result = match selected_id() {
            Some(id) => {
                saving.set(true);
                status.set(String::new());
                let new_password = password();
                let data = UserUpdate {
                    email: Some(trimmed_email),
                    password: (!new_password.is_empty()).then_some(new_password),
                    full_name: Some(name),
                    is_active: Some(is_active()),
                    is_superuser: Some(is_superuser()),
                };
                update_user(&token, &id, &data).await.map(|_| "User updated")
            }
            None => {
                if password().chars().count() < 8 {
                    status.set("Password must be at least 8 characters".into());
                    return;
                }
                saving.set(true);
                status.set(String::new());
                let data = UserCreate {
                    email: trimmed_email,
                    password: password(),
                    full_name: name,
                    is_active: is_active(),
                    is_superuser: is_superuser(),
                };
                create_user(&token, &data).await.map(|_| "User created")
            }
        };
        match result {
            Ok(message) => {
                clear_form();
                status.set(message.into());
                refresh().await;
            }
            Err(err) => fail(err, "Save failed"),
        }
        saving.set(false);
    };

    let on_delete = move |_| async move {
        let (Some(token), Some(id)) = (auth.token(), selected_id()) else { return };
        if auth.state().user.and_then(|u| u.id).as_deref() == Some(id.as_str()) {
            status.set("Cannot delete your own account here".into());
            return;
        }
        saving.set(true);
        status.set(String::new());
        match delete_user(&token, &id).await {
            Ok(()) => {
                clear_form();
                status.set("User deleted".into());
                refresh().await;
            }
            Err(err) => fail(err, "Delete failed"),
        }
        saving.set(false);
    };

    if !auth.state().is_superuser() {
        return rsx! {};
    }

    let editing = selected_id().is_some();
    let status_class = if status().to_lowercase().contains("fail") { "text-sm text-destructive" } else { "text-sm" };

    rsx! {
        div { class: "space-y-6",
            div { class: "flex flex-wrap items-start justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold tracking-tight", "Users" }
                    p { class: "text-muted-foreground", "Manage user accounts and permissions" }
                }
                Button { variant: ButtonVariant::Secondary, onclick: move |_| clear_form(), "Add user" }
            }

            div { class: "grid gap-6 lg:grid-cols-2",
                Card {
                    CardHeader {
                        CardTitle { "All users" }
                        CardDescription { if loading() { "Loading…" } else { "{status}" } }
                    }
                    CardContent { class: "space-y-2",
                        if !loading() && users.read().is_empty() {
                            p { class: "text-sm text-muted-foreground", "No users yet" }
                        } else {
                            for user in users() {
                                Button {
                                    key: "{user.id}",
                                    variant: if selected_id().as_deref() == Some(user.id.as_str()) { ButtonVariant::Default } else { ButtonVariant::Outline },
                                    class: "h-auto w-full justify-start py-2 text-left",
                                    onclick: {
                                        let user = user.clone();
                                        move |_| select_user(user.clone())
                                    },
                                    span { class: "truncate",
                                        "{user.email}"
                                        if user.is_superuser { " (superuser)" }
                                    }
                                }
                            }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { if editing { "Edit user" } else { "New user" } }
                        CardDescription { if editing { "Update account details" } else { "Create a new account" } }
                    }
                    CardContent {
                        form { class: "space-y-4", onsubmit: on_save,
                            div { class: "space-y-2",
                                Label { r#for: "admin-email", "Email" }
                                Input { id: "admin-email", r#type: "email", value: email(), required: true, oninput: move |v| email.set(v) }
                            }
                            div { class: "space-y-2",
                                Label { r#for: "admin-full-name", "Full name" }
                                Input { id: "admin-full-name", value: full_name(), oninput: move |v| full_name.set(v) }
                            }
                            div { class: "space-y-2",
                                Label { r#for: "admin-password",
                                    if editing { "Password (leave blank to keep)" } else { "Password (required)" }
                                }
                                Input {
                                    id: "admin-password",
                                    r#type: "password",
                                    value: password(),
                                    autocomplete: "new-password",
                                    oninput: move |v| password.set(v),
                                }
                            }
                            div { class: "flex items-center justify-between gap-4",
                                Label { r#for: "admin-active", "Is active" }
                                Switch { id: "admin-active", checked: is_active(), onchange: move |v| is_active.set(v) }
                            }
                            div { class: "flex items-center justify-between gap-4",
                                Label { r#for: "admin-superuser", "Is superuser" }
                                Switch { id: "admin-superuser", checked: is_superuser(), onchange: move |v| is_superuser.set(v) }
                            }
                            div { class: "flex flex-wrap gap-2 pt-2",
                                Button { r#type: "submit", disabled: saving(), if saving() { "Saving…" } else { "Save" } }
                                Button {
                                    variant: ButtonVariant::Destructive,
                                    disabled: saving() || !editing,
                                    onclick: on_delete,
                                    "Delete"
                                }
                            }
                            if !status().is_empty() {
                                p { class: "{status_class}", "{status}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

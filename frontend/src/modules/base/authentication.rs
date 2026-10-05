//! Login and dev signup (POST /private/users), the Dioxus port of Rust-Svelte's Authentication.svelte.

use dioxus::prelude::*;

use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::utils::api_error::ApiError;
use crate::modules::base::utils::auth_api::{fetch_current_user, login_with_password, signup_with_private_route};
use crate::routes::Route;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Login,
    Signup,
}

const INPUT: &str = "w-full rounded-lg border border-slate-700 bg-slate-900 px-4 py-3 text-slate-100 placeholder-slate-500 transition focus:border-sky-500 focus:ring-2 focus:ring-sky-500/20 focus:outline-none disabled:opacity-50";

fn valid_email(email: &str) -> bool {
    let email = email.trim();
    match email.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty()
                && !email.contains(char::is_whitespace)
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
        }
        None => false,
    }
}

#[component]
pub fn Authentication() -> Element {
    let auth = use_auth();
    let nav = navigator();

    let mut tab = use_signal(|| Tab::Login);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut full_name = use_signal(String::new);
    let mut error = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let mut switch_tab = move |next: Tab| {
        tab.set(next);
        error.set(String::new());
        email.set(String::new());
        password.set(String::new());
        full_name.set(String::new());
    };

    let after_auth = move |token: String| async move {
        let user = fetch_current_user(&token).await?;
        auth.login(token, user);
        nav.replace(Route::Home {});
        Ok::<(), ApiError>(())
    };

    let on_login = move |evt: FormEvent| async move {
        evt.prevent_default();
        error.set(String::new());
        loading.set(true);
        let result = match login_with_password(&email(), &password()).await {
            Ok(token) => after_auth(token).await,
            Err(err) => Err(err),
        };
        if let Err(err) = result {
            error.set(err.message);
        }
        loading.set(false);
    };

    let on_signup = move |evt: FormEvent| async move {
        evt.prevent_default();
        error.set(String::new());
        if !valid_email(&email()) {
            error.set("Invalid email format".into());
            return;
        }
        if password().chars().count() < 8 {
            error.set("Password must be at least 8 characters long".into());
            return;
        }
        loading.set(true);
        let result = async {
            signup_with_private_route(&email(), &password(), &full_name()).await?;
            let token = login_with_password(&email(), &password()).await?;
            after_auth(token).await
        }
        .await;
        if let Err(err) = result {
            error.set(err.message);
        }
        loading.set(false);
    };

    let busy = loading();
    let login_tab = if tab() == Tab::Login {
        "border-b-2 border-sky-400/70 bg-sky-900/30 text-slate-100"
    } else {
        "text-slate-500 hover:text-slate-300"
    };
    let signup_tab = if tab() == Tab::Signup {
        "border-b-2 border-emerald-400/70 bg-emerald-900/30 text-slate-100"
    } else {
        "text-slate-500 hover:text-slate-300"
    };

    rsx! {
        div { class: "w-full space-y-6",
            div { class: "text-center",
                h2 { class: "text-2xl font-bold tracking-tight text-slate-100", "Welcome back" }
                p { class: "mt-2 font-mono text-xs text-slate-500", "POST /api/v1/private/users/ · dev signup" }
            }

            div { class: "flex border-b border-slate-800",
                button {
                    r#type: "button",
                    class: "flex-1 cursor-pointer py-3 text-sm font-medium transition {login_tab}",
                    onclick: move |_| switch_tab(Tab::Login),
                    "Login"
                }
                button {
                    r#type: "button",
                    class: "flex-1 cursor-pointer py-3 text-sm font-medium transition {signup_tab}",
                    onclick: move |_| switch_tab(Tab::Signup),
                    "Sign up"
                }
            }

            if tab() == Tab::Login {
                form { class: "space-y-4", onsubmit: on_login,
                    input {
                        class: INPUT,
                        r#type: "email",
                        placeholder: "Email",
                        value: "{email}",
                        disabled: busy,
                        required: true,
                        oninput: move |e| email.set(e.value()),
                    }
                    input {
                        class: INPUT,
                        r#type: "password",
                        placeholder: "Password",
                        value: "{password}",
                        disabled: busy,
                        required: true,
                        oninput: move |e| password.set(e.value()),
                    }
                    button {
                        r#type: "submit",
                        disabled: busy,
                        class: "w-full cursor-pointer rounded-lg bg-sky-500 py-3 text-sm font-semibold text-white transition hover:bg-sky-400 disabled:cursor-not-allowed disabled:opacity-50",
                        if busy {
                            span { class: "inline-flex items-center justify-center gap-2",
                                span { class: "size-4 animate-spin rounded-full border-2 border-white/30 border-t-white" }
                                "Logging in…"
                            }
                        } else {
                            "Login"
                        }
                    }
                    p { class: "text-center text-sm text-slate-500",
                        "No account? "
                        button {
                            r#type: "button",
                            class: "cursor-pointer font-medium text-emerald-400 hover:text-emerald-300",
                            onclick: move |_| switch_tab(Tab::Signup),
                            "Sign up"
                        }
                    }
                }
            } else {
                form { class: "space-y-4", onsubmit: on_signup,
                    input {
                        class: INPUT,
                        r#type: "email",
                        placeholder: "Email",
                        value: "{email}",
                        disabled: busy,
                        required: true,
                        oninput: move |e| email.set(e.value()),
                    }
                    input {
                        class: INPUT,
                        r#type: "text",
                        placeholder: "Full name (optional)",
                        value: "{full_name}",
                        disabled: busy,
                        oninput: move |e| full_name.set(e.value()),
                    }
                    input {
                        class: INPUT,
                        r#type: "password",
                        placeholder: "Password (min. 8 characters)",
                        value: "{password}",
                        disabled: busy,
                        required: true,
                        oninput: move |e| password.set(e.value()),
                    }
                    button {
                        r#type: "submit",
                        disabled: busy,
                        class: "w-full cursor-pointer rounded-lg bg-emerald-500 py-3 text-sm font-semibold text-white transition hover:bg-emerald-400 disabled:cursor-not-allowed disabled:opacity-50",
                        if busy {
                            span { class: "inline-flex items-center justify-center gap-2",
                                span { class: "size-4 animate-spin rounded-full border-2 border-white/30 border-t-white" }
                                "Creating account…"
                            }
                        } else {
                            "Create account"
                        }
                    }
                    p { class: "text-center text-sm text-slate-500",
                        "Already registered? "
                        button {
                            r#type: "button",
                            class: "cursor-pointer font-medium text-sky-400 hover:text-sky-300",
                            onclick: move |_| switch_tab(Tab::Login),
                            "Login"
                        }
                    }
                }
            }

            if !error().is_empty() {
                div { class: "rounded-lg border border-red-500/40 bg-red-950/30 px-4 py-3 text-center text-sm text-red-400",
                    "{error}"
                }
            }
        }
    }
}

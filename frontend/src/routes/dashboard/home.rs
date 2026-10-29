use dioxus::prelude::*;
use serde_json::Value;

use crate::config::{api_base_url, platform_name};
use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::ui::{
    Badge, BadgeVariant, Button, Card, CardContent, CardDescription, CardHeader, CardTitle,
};
use crate::modules::base::utils::{auth_api::fetch_current_user, http};

#[component]
pub fn Home() -> Element {
    let auth = use_auth();
    let mut me_check = use_signal(|| "not tested".to_string());
    let mut me_loading = use_signal(|| false);

    let health = use_resource(|| async { http::get_json::<bool>(None, "utils/health-check", "Health check failed").await });
    let sample = use_resource(|| async { http::get_json::<Value>(None, "sample", "Sample request failed").await });

    let test_me = move |_| async move {
        let Some(token) = auth.token() else {
            me_check.set("no token in store".into());
            return;
        };
        me_loading.set(true);
        match fetch_current_user(&token).await {
            Ok(user) => {
                let suffix = if user.is_superuser() { " (superuser)" } else { "" };
                me_check.set(format!("{}{suffix}", user.email));
            }
            Err(err) => me_check.set(err.message),
        }
        me_loading.set(false);
    };

    let (health_variant, health_code, health_text) = match &*health.read() {
        None => (BadgeVariant::Secondary, "…".to_string(), "checking…".to_string()),
        Some(Ok(ok)) => (
            if *ok { BadgeVariant::Default } else { BadgeVariant::Secondary },
            if *ok { "200".into() } else { "503".into() },
            ok.to_string(),
        ),
        Some(Err(err)) => (BadgeVariant::Destructive, "ERR".into(), err.message.clone()),
    };
    let sample_text = match &*sample.read() {
        None => "…".to_string(),
        Some(Ok(body)) => body.get("message").and_then(Value::as_str).unwrap_or("ok").to_string(),
        Some(Err(err)) => err.message.clone(),
    };

    rsx! {
        div { class: "space-y-6",
            div {
                h1 { class: "text-3xl font-bold tracking-tight", "Dashboard" }
                p { class: "text-muted-foreground",
                    "API health checks and session verification. Default superuser: "
                    code { class: "rounded bg-muted px-1.5 py-0.5 text-sm", "admin@example.com" }
                }
            }

            div { class: "grid gap-4 md:grid-cols-2",
                Card {
                    CardHeader {
                        CardTitle { "Authenticated /me" }
                        CardDescription { "Test GET /base/login/me with stored JWT" }
                    }
                    CardContent { class: "space-y-4",
                        Button { disabled: me_loading(), onclick: test_me,
                            if me_loading() { "Calling /me…" } else { "Test GET /base/login/me" }
                        }
                        p { class: "font-mono text-sm text-muted-foreground", "{me_check}" }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Health check" }
                        CardDescription { "GET /api/v1/utils/health-check/" }
                    }
                    CardContent {
                        div { class: "flex items-center gap-2",
                            Badge { variant: health_variant, "{health_code}" }
                            span { class: "font-mono text-sm", "{health_text}" }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Sample module" }
                        CardDescription { "GET /api/v1/sample/" }
                    }
                    CardContent {
                        p { class: "font-mono text-sm", "{sample_text}" }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Client" }
                        CardDescription { "Platform this build runs on and the API it calls" }
                    }
                    CardContent { class: "space-y-2",
                        div { class: "flex items-center gap-2",
                            Badge { variant: BadgeVariant::Outline, "{platform_name()}" }
                        }
                        p { class: "font-mono text-sm break-all text-muted-foreground", "{api_base_url()}" }
                    }
                }
            }
        }
    }
}

//! Web only: installable PWA (manifest + service worker from `frontend/assets`). No-op on native clients.

use dioxus::prelude::*;

#[component]
pub fn PwaRegister() -> Element {
    if !cfg!(target_arch = "wasm32") {
        return rsx! {};
    }
    use_effect(|| {
        document::eval(
            "if ('serviceWorker' in navigator) { navigator.serviceWorker.register('/sw.js').catch(() => {}); }",
        );
    });
    rsx! {
        document::Link { rel: "manifest", href: "/manifest.webmanifest" }
        document::Link { rel: "apple-touch-icon", href: "/icons/apple-touch-icon.png" }
        document::Meta { name: "theme-color", content: "#09090b" }
    }
}

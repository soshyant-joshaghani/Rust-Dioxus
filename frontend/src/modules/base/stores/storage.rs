//! Small persistent key/value store, the `localStorage` of every client.
//! - web: `window.localStorage`
//! - desktop / iOS: `<data dir>/rust-dioxus/storage.json`
//! - Android: the app's private `files/` directory

#[cfg(target_arch = "wasm32")]
mod imp {
    fn local() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    pub fn get(key: &str) -> Option<String> {
        local()?.get_item(key).ok().flatten()
    }

    pub fn set(key: &str, value: &str) {
        if let Some(storage) = local() {
            let _ = storage.set_item(key, value);
        }
    }

    pub fn remove(key: &str) {
        if let Some(storage) = local() {
            let _ = storage.remove_item(key);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Must match `[bundle] identifier` in Dioxus.toml.
    #[cfg(target_os = "android")]
    const ANDROID_APP_ID: &str = "rustdioxus.client";

    fn dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from(format!("/data/data/{ANDROID_APP_ID}/files"))
        }
        #[cfg(not(target_os = "android"))]
        {
            dirs::data_local_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("rust-dioxus")
        }
    }

    fn file() -> PathBuf {
        dir().join("storage.json")
    }

    fn load() -> BTreeMap<String, String> {
        std::fs::read_to_string(file())
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default()
    }

    fn save(map: &BTreeMap<String, String>) {
        let _ = std::fs::create_dir_all(dir());
        if let Ok(raw) = serde_json::to_string_pretty(map) {
            let _ = std::fs::write(file(), raw);
        }
    }

    pub fn get(key: &str) -> Option<String> {
        load().get(key).cloned()
    }

    pub fn set(key: &str, value: &str) {
        let mut map = load();
        map.insert(key.to_string(), value.to_string());
        save(&map);
    }

    pub fn remove(key: &str) {
        let mut map = load();
        if map.remove(key).is_some() {
            save(&map);
        }
    }
}

pub use imp::{get, remove, set};

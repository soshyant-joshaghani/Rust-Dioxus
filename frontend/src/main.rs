use rust_dioxus_client::{modules::base::APP_NAME, App};

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

        let window = WindowBuilder::new()
            .with_title(APP_NAME)
            .with_inner_size(LogicalSize::new(1280.0, 820.0))
            .with_min_inner_size(LogicalSize::new(380.0, 600.0));
        dioxus::LaunchBuilder::desktop()
            .with_cfg(Config::new().with_window(window))
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    {
        let _ = APP_NAME;
        dioxus::launch(App);
    }
}

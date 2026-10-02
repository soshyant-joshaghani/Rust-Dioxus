use rust_dioxus::core::config::Settings;
use rust_dioxus::core::init_tracing;
use rust_dioxus::core::jobs::run_worker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let settings = Settings::from_env()?;
    run_worker(&settings.redis_url()).await
}

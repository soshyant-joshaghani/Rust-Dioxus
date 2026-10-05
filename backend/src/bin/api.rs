use std::sync::Arc;

use rust_dioxus::build_router;
use rust_dioxus::core::cache::{Cache, RedisCache, RedisPool};
use rust_dioxus::core::config::Settings;
use rust_dioxus::core::db;
use rust_dioxus::core::jobs::{JobQueue, RedisJobQueue};
use rust_dioxus::core::state::AppState;
use rust_dioxus::core::{init_tracing, shutdown_signal};
use rust_dioxus::modules::apps::sample::repository::{NoteRepository, PgNoteRepository};
use rust_dioxus::modules::base::users::repository::{PgUserRepository, UserRepository};
use rust_dioxus::modules::base::users::service::UserService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let settings = Settings::from_env()?;
    settings.validate()?;

    let pool = db::connect_pool(&settings).await?;
    match db::find_migrations_dir(settings.migrations_dir.as_deref()) {
        Some(dir) => db::run_migrations(&pool, &dir).await?,
        None => anyhow::bail!("migrations directory not found (set MIGRATIONS_DIR)"),
    }

    let redis = Arc::new(RedisPool::new(&settings.redis_url())?);
    let cache: Arc<dyn Cache> = Arc::new(RedisCache::new(redis.clone()));
    let jobs: Arc<dyn JobQueue> = Arc::new(RedisJobQueue::new(redis));
    let users: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(pool.clone()));
    let notes: Arc<dyn NoteRepository> = Arc::new(PgNoteRepository::new(pool));

    let state = AppState::new(Arc::new(settings.clone()), users, notes, cache, jobs);

    UserService::new(&state)
        .ensure_first_superuser(&settings.first_superuser, &settings.first_superuser_password)
        .await?;

    let app = build_router(state);

    let addr = format!("{}:{}", settings.app_host, settings.app_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(
        "{} listening on {} (environment: {})",
        settings.project_name,
        addr,
        settings.environment
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    tracing::info!("server stopped");
    Ok(())
}

use crate::app::{config::Config, shutdown::shutdown_signal};
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use anyhow::{Context, Result};
use axum::{
    Router,
    routing::get,
    http::StatusCode,
};
use axum::response::IntoResponse;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    pub async fn run(&mut self) -> Result<()> {
        println!();
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message("Starting...");

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .connect(self.config.database_url.as_str())
            .await
            .context("failed to connect to database")?;

        let spa_service = ServeDir::new("./static")
            .not_found_service(tower_http::services::ServeFile::new("./static/index.html"));

        let api_router = Router::new()
            .route("/servers",
                   get(crate::handles::list::get_servers_list::get_servers_list)
                       .post(crate::handles::list::register_server::register_server)
            )
            .route("/servers/{id}",
                   get(crate::handles::list::get_server_info::get_server_info)
                       .put(crate::handles::list::edit_server_info::edit_server_info)
                       .delete(crate::handles::list::delete_server::delete_server)
            )
            .route("/servers/{id}/health", get(crate::handles::manage::health::get_server_health))
            .route("/servers/{id}/specs", get(crate::handles::manage::specs::get_server_specs));

        let app = Router::new()
            .nest("/api/v1", api_router)
            .fallback_service(spa_service)
            .layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
            ))
            .with_state(pool);

        let listener =
            TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, self.config.server.port.clone()))).await?;

        pb.finish_and_clear();
        println!("{}", format!("{} Ready!\n", "✔".green()));

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("failed to start server")?;

        info!("Server shutting down gracefully.");
        Ok(())
    }
}

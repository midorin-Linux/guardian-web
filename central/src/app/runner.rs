
use crate::app::{config::Config, shutdown::shutdown_signal};
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use anyhow::{Context, Result};
use axum::{
    Router,
    routing::{get, post},
    http::StatusCode,
};
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

        let app = Router::new()
            .route("/api/register", post(crate::handles::register::server_register))
            .route("/api/server-list", get(crate::handles::server_list::find_server_list))
            // React側でのルーティングを許可する
            .nest_service("/info", ServeDir::new("./static").append_index_html_on_directories(true).not_found_service(ServeDir::new("./static")))
            .nest_service("/list", ServeDir::new("./static").append_index_html_on_directories(true).not_found_service(ServeDir::new("./static")))
            .nest_service("/monitor", ServeDir::new("./static").append_index_html_on_directories(true).not_found_service(ServeDir::new("./static")))
            .fallback_service(ServeDir::new("./static").append_index_html_on_directories(true).not_found_service(ServeDir::new("./static")))
            .layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
            ))
            .with_state(pool);

        let listener =
            TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, self.config.port.clone()))).await?;

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

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Sqlite>);
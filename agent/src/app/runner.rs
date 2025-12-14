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
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use tokio::net::TcpListener;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
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

        let api_router = Router::new()
            .route("/metrics", get(crate::handles::metrics::sse_handler))
            .route("/info", get(crate::handles::info::get_server_information));

        let app = Router::new()
            .nest("/api/agent/v1", api_router)
            .layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
            ));

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

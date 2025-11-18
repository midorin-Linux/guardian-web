use crate::app::{config::Config, shutdown::shutdown_signal};
use crate::handles::{specifications, stream_monitor};
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{FromRequestParts, State},
    http::{StatusCode, request::Parts},
    middleware as axum_middleware,
    response::{Html, IntoResponse, Json, Response},
    routing::{delete, get, post, put},
};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

pub struct App {
    port: u16,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { port: config.port })
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

        let app = Router::new()
            .layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::new(Duration::from_secs(10)),
            ))
            .route("/api/device-components", get(specifications::get_full_spec))
            .route("/api/monitor/stream", get(stream_monitor::sse_handler))
            // React側でのルーティングを許可する
            .nest_service("/info", ServeDir::new("./static").append_index_html_on_directories(true).not_found_service(ServeDir::new("./static")))
            .nest_service("/monitor", ServeDir::new("./static").append_index_html_on_directories(true).not_found_service(ServeDir::new("./static")))
            .fallback_service(ServeDir::new("./static").append_index_html_on_directories(true).not_found_service(ServeDir::new("./static")));

        let listener =
            TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, self.port.clone()))).await?;

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

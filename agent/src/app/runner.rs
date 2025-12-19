use crate::app::{config::Config, shutdown::shutdown_signal};
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use anyhow::{Context, Result};
use axum::{
    extract::ConnectInfo,
    middleware::Next,
    response::{IntoResponse, Response},
    Router,
    routing::get,
    http::StatusCode,
};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::{debug, error, info, warn};

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
            .route("/health", get(StatusCode::OK))
            .route("/metrics", get(crate::handles::metrics::sse_handler))
            .route("/info", get(crate::handles::info::get_server_information));

        let app = Router::new()
            .nest("/api/agent/v1", api_router)
            .layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
                axum::middleware::from_fn(whitelist)
            ));

        let listener =
            TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, self.config.server.port.clone()))).await?;

        pb.finish_and_clear();
        println!("{}", format!("{} Ready!\n", "✔".green()));

        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>()
        )
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("failed to start server")?;

        info!("Server shutting down gracefully.");
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Whitelist {
    ip_address: String,
}

async fn whitelist(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    debug!("Request from: {}", addr.ip());

    let json_row = match std::fs::read_to_string("whitelist.json") {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to read whitelist.json: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    let whitelist: Vec<Whitelist> = match serde_json::from_str(&json_row) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to parse whitelist.json: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    let client_ip = addr.ip().to_string();
    if !whitelist.iter().any(|data| data.ip_address.eq(&client_ip)) {
        warn!("Access denied for IP: {}", client_ip);
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    next.run(request).await
}

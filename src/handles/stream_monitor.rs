use axum::{
    extract::Query,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use sysinfo::System;

#[derive(Deserialize)]
pub struct StreamComponent {
    #[serde(default = "default_interval")]
    pub interval: u64,
}

#[derive(Serialize, Debug)]
pub struct ResponseJson {
    cpu: f32,
    ram: f32,
}

fn default_interval() -> u64 {
    1000
}

pub async fn sse_handler(Query(params): Query<StreamComponent>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel::<String>(16);
    let _handle = tokio::spawn(async move {
        let mut sys = System::new_all();
        let mut interval = tokio::time::interval(Duration::from_millis(params.interval));
        loop {
            interval.tick().await;
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
            let ram_usage = sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0;

            let json = ResponseJson {
                cpu: (cpu_usage * 100.0).round() / 100.0,
                ram: (ram_usage * 100.0).round() / 100.0,
            };

            match serde_json::to_string(&json) {
                Ok(msg) => {
                    tracing::debug!("Sending JSON: {}", msg);
                    if tx.send(msg).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to serialize JSON: {}", e);
                    break;
                }
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx)
        .map(|msg| Ok(Event::default().data(msg)))
        .throttle(Duration::from_millis(100));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
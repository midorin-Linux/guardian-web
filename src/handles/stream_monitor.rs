use axum::{
    extract::Query,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use serde::Deserialize;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use sysinfo::System;

#[derive(Deserialize)]
pub struct StreamComponent {
    pub component: String,
    #[serde(default = "default_interval")]
    pub interval: u64,
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
            let mut msg = String::new();
            match params.component.as_str() {
                // CPU使用率(パーセンテージで表示)
                "cpu" => {
                    sys.refresh_cpu_usage();
                    let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
                    msg = format!("{:.2}", cpu_usage);
                }
                // RAM使用率(GBで表示)
                "ram" => {
                    sys.refresh_memory();
                    let ram_usage = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
                    msg = format!("{:.2}", ram_usage);
                }
                _ => {
                    break;
                }
            }

            if tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx)
        .map(|msg| Ok(Event::default().data(msg)))
        .throttle(Duration::from_millis(100));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
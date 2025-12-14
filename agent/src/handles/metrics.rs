use common::agent::metrics::*;

use anyhow::Result;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use sysinfo::{Disks, System};

pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel::<String>(16);
    let _handle = tokio::spawn(async move {
        let mut sys = System::new_all();
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        loop {
            interval.tick().await;
            
            let cpu_metrics = get_cpu_metrics(&mut sys).await;
            let memory_metrics = get_memory_metrics(&mut sys).await;
            let disk_metrics = get_disk_metrics().await;
            
            let json = ServerMetrics {
                cpu: cpu_metrics.unwrap(),
                memory: memory_metrics.unwrap(),
                disk: disk_metrics.unwrap(),
                uptime_seconds: 114514, //ToDo: Uptimeの処理を作る
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

async fn get_cpu_metrics(sys: &mut System) -> Result<Cpu> {
    sys.refresh_cpu_usage();
    let usage_percent = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    let cores = System::physical_core_count().unwrap() as u64;
    let threads = sys.cpus().len() as u64;
    
    Ok(Cpu {
        usage_percent,
        cores,
        threads,
    })
}

async fn get_memory_metrics(sys: &mut System) -> Result<Memory> {
    sys.refresh_memory();
    Ok(Memory {
        total_bytes: sys.total_memory(),
        used_bytes: sys.used_memory(),
        free_bytes: sys.free_memory(),
    })
}

async fn get_disk_metrics() -> Result<Vec<Disk>> {
    let mut disks = Disks::new();
    disks.refresh(true);
    
    let storage = disks
        .iter()
        .map(|disk| Disk {
            mount: disk.mount_point().to_string_lossy().to_string(),
            total_bytes: disk.total_space(),
            used_bytes: disk.total_space() - disk.available_space(),
            free_bytes: disk.available_space(),
            device: disk.name().to_string_lossy().to_string(),
        })
        .collect::<Vec<Disk>>();
    Ok(storage)
}
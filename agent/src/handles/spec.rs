use crate::handles::get_spec::get_spec::GetSpec;
use common::FullSpec;

use axum::response::{IntoResponse, Json};

pub async fn get_full_spec() -> impl IntoResponse {
    let mut get_spec = GetSpec::new().unwrap();
    let device = get_spec.device().await.unwrap();
    let cpu = get_spec.cpu().await.unwrap();
    let ram = get_spec.ram().await.unwrap();
    let storage = get_spec.storage().await.unwrap();
    let gpu = get_spec.gpu().await.unwrap();

    let spec = Json(FullSpec {
        device,
        cpu,
        ram,
        storage,
        gpu,
    });

    spec
}

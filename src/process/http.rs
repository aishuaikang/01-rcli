use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", dir, addr);

    let state = HttpServeState { dir };

    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.dir).join(path);

    if !p.is_file() {
        warn!("File not found: {:?}", p);
        return (StatusCode::NOT_FOUND, "Not Found".to_string());
    }

    info!("Reading file {:?}", p);
    match tokio::fs::read_to_string(p).await {
        Ok(content) => (StatusCode::OK, content),
        Err(err) => {
            warn!("Error reading file: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading file: {:?}", err),
            )
        }
    }
}

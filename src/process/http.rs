use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::fs::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", dir, addr);

    let state = HttpServeState { dir: dir.clone() };

    let dir_service = ServeDir::new(dir)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate();

    let router = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let p = std::path::Path::new(&state.dir).join(path);

    if !p.is_file() {
        if !p.is_dir() {
            warn!("File not found: {:?}", p);
            return (StatusCode::NOT_FOUND, "Not Found".to_string()).into_response();
        }

        let a = p.read_dir().unwrap();
        return Html(format!(
            "<html><body><h1>Directory listing for {:?}</h1><ul>{}</ul></body></html>",
            p.display(),
            a.map(|item| {
                let item = item.unwrap();

                let href = p.join(item.file_name()).display().to_string();
                let file_name = item.file_name().into_string().unwrap();
                format!(r#"<li><a href="{}">{}</a></li>"#, href, file_name)
            })
            .collect::<Vec<String>>()
            .join("\n")
        ))
        .into_response();
    }

    info!("Reading file {:?}", p);
    match tokio::fs::read_to_string(p).await {
        Ok(content) => (StatusCode::OK, content).into_response(),
        Err(err) => {
            warn!("Error reading file: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading file: {:?}", err),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            dir: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());

        let result = file_handler(State(state), path).await.into_response();

        assert_eq!(result.status(), StatusCode::OK);
    }
}

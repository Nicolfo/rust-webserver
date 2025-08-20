use axum::body::Body;
use axum::{
    Router,
    extract::{OriginalUri, State},
    http::{HeaderMap, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use bytes::Bytes;
use mime_guess::from_path;
use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, RwLock},
};
use tokio::fs;

const SINGLE_PAGE: bool = true;

#[derive(Debug)]
struct AppState {
    file_cache: RwLock<HashMap<String, Bytes>>, // Store binary content
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        file_cache: RwLock::new(HashMap::new()),
    });

    let app = Router::new()
        .fallback(fallback_handler)
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fallback_handler(
    OriginalUri(uri): OriginalUri,
    State(state): State<Arc<AppState>>,
) -> Response<Body> {
    let path = if uri.path().ends_with("/") {
        format!("{}index.html", uri.path())
    } else {
        uri.path().to_string()
    };

    // Check cache
    if let Some(content) = state.file_cache.read().unwrap().get(path.as_str()) {
        println!("Request path {}, Serving from cache, rensponse size is {}",&path, content.len());
        return build_response(path.as_str(), content.clone());
    }

    // Try to load from filesystem
    let file_path = PathBuf::from(format!("static{}", path));
    match fs::read(&file_path).await {
        Ok(bytes) => {
            let content = Bytes::from(bytes);
            state
                .file_cache
                .write()
                .unwrap()
                .insert(path.to_string(), content.clone());
            println!("Request path {}, Serving from filesystem, rensponse size is {}",&file_path.to_str().unwrap() , content.len());
            build_response(path.as_str(), content)
        }
        Err(_) => {
            if SINGLE_PAGE {
                // Try to serve index.html instead
                let index_path = PathBuf::from("static/index.html");
                match fs::read(&index_path).await {
                    Ok(bytes) => {
                        let content = Bytes::from(bytes);
                        // Cache the index.html content for future requests
                        state
                            .file_cache
                            .write()
                            .unwrap()
                            .insert(String::from("/index.html"), content.clone());
                        println!("Request path {}, Serving index.html from filesystem, rensponse size is {}",&file_path.to_str().unwrap() , content.len());
                        build_response("/index.html", content)
                    }
                    Err(_) => {
                        println!("File not found: {}", path);
                        (StatusCode::NOT_FOUND, format!("File not found: {}", path)).into_response()
                    }
                }
            } else {
                println!("File not found: {}", path);
                (StatusCode::NOT_FOUND, format!("File not found: {}", path)).into_response()
            }
        }
    }
}

fn build_response(path: &str, content: Bytes) -> Response<Body> {
    let mime = from_path(path).first_or_octet_stream();

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_str(mime.as_ref())
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );

    (headers, content).into_response()
}

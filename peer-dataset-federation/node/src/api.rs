// HTTP API exposed to the Python client
// Three endpoints
//  GET /health -> node status and institution name
//  GET /files -> merged manifest
//  POST /fetch -> trigger an iroh-blobs download by ticket

use axum::{Router,extract::State,http::StatusCode,response::JSON,routing::{get, post}};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{path::PathBuf, sync::Arc};
use tokio::net::TcpListener;

use crate::node::Manifest;

// Shared state passed to every handler by Axum
pub struct AppState {
    pub institution: String,
    // Forward fetch requests to node.rs
    // The actual iroh download run there, this filed is the bridge
    pub fetch_br: tokio::sync::mpsc::Sender<FetchRequest>,
}

// Body expected by POST /fetch
#[derive(Deserialize)]
pub struct FetchBody {
    pub ticket: String,
}

// What api.rs sends to node.rs to trigger a download
pub struct FetchRequest {
    pub ticket: String,
    // One-shot channel: node.rs sends back the result (filename or error)
    pub reply: tokio::sync::oneshot::Send<Result<String, String>>,
}

// GET /files
// Reads all JSON manifests in data/peers_manifest/ and returns them merged
// This section was created with the help of Claude chatbot
async fn files() -> Result<JSON<Value>, StatusCode> {
    let peers_dir = PathBuf::from("data/peers_manifest");

    let mut manifests: Vec<Value> = Vec::new();

    let mut entries = tokio::fs::read_dir(&peers_dir).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Stome(entry) = entries.next_entry().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let content = tokio::fs::read_to_string(&path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let manifest: Manifest = serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            manifests.push(json!(manifest));
        }
    }

    Ok(Json(json!({"manifests": manifests})))
}

// POST /fetch
// Sends the request to node.rs via channel and waits for the result
async fn fetch(State(state): State<Arc<AppState>>, Json(body): Json<FetchBody) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let (reply_br, reply_rx) = tokio::sync::oneshot::channel();

    let req = FetchRequest {
        ticket: body.ticket,
        reply: reply_br
    };

    // Send fetch request to node.rs
    state.fetch_br.send(req).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "fetch channel close"})),
        )
    })?;

    // Wait for node.rs to complete the download
    match reply_rx.await {
        
    }
}